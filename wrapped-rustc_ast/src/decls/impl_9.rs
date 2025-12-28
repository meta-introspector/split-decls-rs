macro_rules! deps {
    () => {
        Path!();
        HashStableContext!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < CTX : rustc_span :: HashStableContext > HashStable < CTX > for Path { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . segments . len () . hash_stable (hcx , hasher) ; for segment in & self . segments { segment . ident . hash_stable (hcx , hasher) ; } } }
    };
}

impl_9!()