macro_rules! deps {
    () => {
        FnvHasher!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Default for FnvHasher { # [inline] fn default () -> Self { Self { state : BASIS } } }
    };
}

impl_3!()