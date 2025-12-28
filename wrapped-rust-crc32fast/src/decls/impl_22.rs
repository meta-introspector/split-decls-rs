macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Default for Hasher { fn default () -> Self { Self :: new () } }
    };
}

impl_22!();