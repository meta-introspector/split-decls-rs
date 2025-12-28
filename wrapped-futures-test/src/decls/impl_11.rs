macro_rules! deps {
    () => {
        NoopSpawner!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl NoopSpawner { # [doc = " Create a new instance"] pub fn new () -> Self { Self { _reserved : () } } }
    };
}

impl_11!()