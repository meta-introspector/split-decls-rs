macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T , S > fmt :: Debug for Intersection < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_171!()