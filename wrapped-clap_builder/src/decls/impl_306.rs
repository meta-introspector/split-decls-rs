macro_rules! deps {
    () => {
        FalseyValueParser!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl Default for FalseyValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_306!();