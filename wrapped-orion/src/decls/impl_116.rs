macro_rules! deps {
    () => {
        Sha256!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl Default for Sha256 { fn default () -> Self { Self :: new () } }
    };
}

impl_116!()