macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl < T , S , A > fmt :: Debug for Difference < '_ , T , S , A > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_447!();