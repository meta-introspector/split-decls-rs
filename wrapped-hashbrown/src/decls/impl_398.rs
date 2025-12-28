macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl < T , S , A > fmt :: Debug for HashSet < T , S , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
    };
}

impl_398!()