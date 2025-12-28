macro_rules! deps {
    () => {
        OwnerNodes!();
        HashStableContext!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < 'tcx , HirCtx : crate :: HashStableContext > HashStable < HirCtx > for OwnerNodes < 'tcx > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let OwnerNodes { opt_hash_including_bodies , nodes : _ , bodies : _ } = * self ; opt_hash_including_bodies . unwrap () . hash_stable (hcx , hasher) ; } }
    };
}

impl_463!();