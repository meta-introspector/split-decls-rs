macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < T , S > fmt :: Debug for Union < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_179!()