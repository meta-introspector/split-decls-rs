macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for Hasher { fn default () -> Self { Self :: new () } }
    };
}

impl_10!()