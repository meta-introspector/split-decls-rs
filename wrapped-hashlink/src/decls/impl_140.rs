macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < T , S > fmt :: Debug for LinkedHashSet < T , S > where T : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
    };
}

impl_140!()