macro_rules! deps {
    () => {
        Lazy!();
    };
}

macro_rules! impl_680 {
    () => {
        deps!();
        impl < T : fmt :: Debug , F : Fn () -> T > fmt :: Debug for Lazy < T , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_680!()