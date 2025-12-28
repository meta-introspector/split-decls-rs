macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Default for Hasher { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_205!()