macro_rules! deps {
    () => {
        Sha3_512!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl Default for Sha3_512 { fn default () -> Self { Self :: new () } }
    };
}

impl_193!();