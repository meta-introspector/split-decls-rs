macro_rules! deps {
    () => {
        Pointer!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl Default for Pointer { # [inline] fn default () -> Self { Pointer :: Direct (0) } }
    };
}

impl_252!();