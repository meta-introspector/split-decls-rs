macro_rules! deps {
    () => {
        StringValueParser!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl Default for StringValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_272!()