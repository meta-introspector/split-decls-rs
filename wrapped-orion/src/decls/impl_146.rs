macro_rules! deps {
    () => {
        Sha512!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Default for Sha512 { fn default () -> Self { Self :: new () } }
    };
}

impl_146!();