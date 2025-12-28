macro_rules! deps {
    () => {
        TokenStream!();
        HashStableContext!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for TokenStream where CTX : crate :: HashStableContext , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { for sub_tt in self . iter () { sub_tt . hash_stable (hcx , hasher) ; } } }
    };
}

impl_449!()