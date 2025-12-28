macro_rules! deps {
    () => {
        PanicSpawner!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl PanicSpawner { # [doc = " Create a new instance"] pub fn new () -> Self { Self { _reserved : () } } }
    };
}

impl_17!();