macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < C > HashStable < C > for ExternAbi { # [inline] fn hash_stable (& self , _ : & mut C , hasher : & mut StableHasher) { Hash :: hash (self , hasher) ; } }
    };
}

impl_23!();