macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for LazyAttrTokenStream { fn hash_stable (& self , _hcx : & mut CTX , _hasher : & mut StableHasher) { panic ! ("Attempted to compute stable hash for LazyAttrTokenStream") ; } }
    };
}

impl_428!()