macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl Default for Config { fn default () -> Config { Config :: new () } }
    };
}

impl_107!();