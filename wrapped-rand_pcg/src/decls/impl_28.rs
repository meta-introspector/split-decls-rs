macro_rules! deps {
    () => {
        Lcg64Xsh32!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Debug for Lcg64Xsh32 { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Lcg64Xsh32 {{}}") } }
    };
}

impl_28!();