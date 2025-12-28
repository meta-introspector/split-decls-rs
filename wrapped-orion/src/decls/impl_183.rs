macro_rules! deps {
    () => {
        Sha3_384!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl Default for Sha3_384 { fn default () -> Self { Self :: new () } }
    };
}

impl_183!()