macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Default for Version { fn default () -> Self { Version :: Unknown } }
    };
}

impl_53!()