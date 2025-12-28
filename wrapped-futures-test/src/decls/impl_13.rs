macro_rules! deps {
    () => {
        NoopSpawner!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Default for NoopSpawner { fn default () -> Self { Self :: new () } }
    };
}

impl_13!();