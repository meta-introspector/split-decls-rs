macro_rules! deps {
    () => {
        HashStableContext!();
        HashIgnoredAttrId!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for HashIgnoredAttrId { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { hcx . hash_attr_id (self , hasher) } }
    };
}

impl_467!();