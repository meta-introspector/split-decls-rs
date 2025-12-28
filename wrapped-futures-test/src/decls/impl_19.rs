macro_rules! deps {
    () => {
        PanicSpawner!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Default for PanicSpawner { fn default () -> Self { Self :: new () } }
    };
}

impl_19!();