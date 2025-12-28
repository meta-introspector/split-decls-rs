macro_rules! deps {
    () => {
        Config!();
        Default!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl Default for Config { fn default () -> Self { Self :: all () } }
    };
}

impl_506!()