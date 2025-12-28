macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < T , S > fmt :: Debug for Difference < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_174!();