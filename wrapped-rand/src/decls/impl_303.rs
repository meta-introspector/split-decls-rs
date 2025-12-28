macro_rules! deps {
    () => {
        RngReader!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < R : TryRngCore > std :: fmt :: Debug for RngReader < R > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("RngReader") . finish () } }
    };
}

impl_303!();