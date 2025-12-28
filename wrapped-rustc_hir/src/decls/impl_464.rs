macro_rules! deps {
    () => {
        DelayedLints!();
        HashStableContext!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for DelayedLints { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let DelayedLints { opt_hash , .. } = * self ; opt_hash . unwrap () . hash_stable (hcx , hasher) ; } }
    };
}

impl_464!()