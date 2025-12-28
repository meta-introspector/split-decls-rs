macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Default for Hash { fn default () -> Self { Self :: new () } }
    };
}

impl_55!()