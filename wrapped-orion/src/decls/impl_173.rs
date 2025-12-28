macro_rules! deps {
    () => {
        Sha3_256!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl Default for Sha3_256 { fn default () -> Self { Self :: new () } }
    };
}

impl_173!();