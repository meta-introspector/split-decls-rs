macro_rules! deps {
    () => {
        Sha3_224!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Default for Sha3_224 { fn default () -> Self { Self :: new () } }
    };
}

impl_163!();