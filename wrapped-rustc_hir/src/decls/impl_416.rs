macro_rules! impl_416 {
    () => {
        impl < CTX > HashStable < CTX > for LangItem { fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
    };
}

impl_416!()