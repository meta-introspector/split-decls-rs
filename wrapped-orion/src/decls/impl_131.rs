macro_rules! deps {
    () => {
        Sha384!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Default for Sha384 { fn default () -> Self { Self :: new () } }
    };
}

impl_131!();