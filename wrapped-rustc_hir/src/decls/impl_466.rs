macro_rules! deps {
    () => {
        Crate!();
        HashStableContext!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for Crate < '_ > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let Crate { owners : _ , opt_hir_hash } = self ; opt_hir_hash . unwrap () . hash_stable (hcx , hasher) } }
    };
}

impl_466!()