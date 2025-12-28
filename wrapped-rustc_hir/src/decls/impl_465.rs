macro_rules! deps {
    () => {
        AttributeMap!();
        HashStableContext!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < 'tcx , HirCtx : crate :: HashStableContext > HashStable < HirCtx > for AttributeMap < 'tcx > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let AttributeMap { opt_hash , define_opaque : _ , map : _ } = * self ; opt_hash . unwrap () . hash_stable (hcx , hasher) ; } }
    };
}

impl_465!()