macro_rules! deps {
    () => {
        Default!();
        Figure!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Default for Figure { fn default () -> Self { Self :: new () } }
    };
}

impl_24!()