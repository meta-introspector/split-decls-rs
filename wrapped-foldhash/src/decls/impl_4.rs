macro_rules! deps {
    () => {
        RandomState!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Default for RandomState { # [inline (always)] fn default () -> Self { Self { per_hasher_seed : gen_per_hasher_seed () , global_seed : GlobalSeed :: new () , } } }
    };
}

impl_4!()