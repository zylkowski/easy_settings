use std::{any::Any, collections::HashMap};

#[macro_export]
macro_rules! settings {
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Settings::new(Iterator::collect(IntoIterator::into_iter([$(($k, Box::new($v) as Box<dyn Any>),)*])))
    }};
}

pub struct Settings {
    settings: HashMap<&'static str, Box<dyn Any>>,
}

impl Settings {
    pub fn new(settings: HashMap<&'static str, Box<dyn Any>>) -> Self {
        Self { settings }
    }

    pub fn get_setting_mut<T: 'static>(&mut self, name: &'static str) -> Option<&mut T> {
        self.settings.get_mut(name)?.downcast_mut::<T>()
    }

    pub fn get_setting<T: 'static>(&self, name: &'static str) -> Option<&T> {
        self.settings.get(name)?.downcast_ref::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let test_settings = settings! {
            "a" => 5,
            "b" => 12.4,
            "c" => "aaa",
            "nested" => settings!{
                "bla" => "no problemo"
            }
        };

        assert!(*test_settings.get_setting::<i32>("a").unwrap() == 5);
        assert!(*test_settings.get_setting::<f64>("b").unwrap() == 12.4);
        assert!(*test_settings.get_setting::<&'static str>("c").unwrap() == "aaa");
    }
}
