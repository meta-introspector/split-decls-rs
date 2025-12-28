macro_rules! deps {
    () => {
        HashTable!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < T , A > fmt :: Debug for HashTable < T , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
    };
}

impl_476!();