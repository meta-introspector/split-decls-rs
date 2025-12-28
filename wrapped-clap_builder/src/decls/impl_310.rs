macro_rules! deps {
    () => {
        BoolishValueParser!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl Default for BoolishValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_310!()