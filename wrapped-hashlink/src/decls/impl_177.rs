macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T , S > fmt :: Debug for SymmetricDifference < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_177!()