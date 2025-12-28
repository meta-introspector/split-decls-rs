macro_rules! deps {
    () => {
        BoolValueParser!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl Default for BoolValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_302!()