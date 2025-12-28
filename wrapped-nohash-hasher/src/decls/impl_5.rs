macro_rules! deps {
    () => {
        NoHashHasher!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T > fmt :: Debug for NoHashHasher < T > { # [cfg (debug_assertions)] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("NoHashHasher") . field (& self . 0) . field (& self . 1) . finish () } # [cfg (not (debug_assertions))] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("NoHashHasher") . field (& self . 0) . finish () } }
    };
}

impl_5!();