macro_rules! deps {
    () => {
        HashTable!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < T , A > Clone for HashTable < T , A > where T : Clone , A : Allocator + Clone , { fn clone (& self) -> Self { Self { raw : self . raw . clone () , } } }
    };
}

impl_475!();