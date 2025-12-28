macro_rules! deps {
    () => {
        NoHashHasher!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T > Clone for NoHashHasher < T > { # [cfg (debug_assertions)] fn clone (& self) -> Self { NoHashHasher (self . 0 , self . 1 , self . 2) } # [cfg (not (debug_assertions))] fn clone (& self) -> Self { NoHashHasher (self . 0 , self . 1) } }
    };
}

impl_7!();