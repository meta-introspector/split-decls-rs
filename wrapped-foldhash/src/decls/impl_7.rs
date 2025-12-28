macro_rules! deps {
    () => {
        SeedableRandomState!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for SeedableRandomState { # [inline (always)] fn default () -> Self { Self :: random () } }
    };
}

impl_7!()