macro_rules! deps {
    () => {
        HashStableContext!();
        DiagnosticItems!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < CTX : crate :: HashStableContext > HashStable < CTX > for DiagnosticItems { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . name_to_id . hash_stable (ctx , hasher) ; } }
    };
}

impl_101!();