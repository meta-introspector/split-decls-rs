macro_rules! deps {
    () => {
        FxRandomState!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for FxRandomState { fn default () -> Self { Self :: new () } }
    };
}

impl_7!()