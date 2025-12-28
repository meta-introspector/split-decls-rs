macro_rules! deps {
    () => {
        FixedState!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Default for FixedState { # [inline (always)] fn default () -> Self { Self { per_hasher_seed : ARBITRARY3 , } } }
    };
}

impl_12!()