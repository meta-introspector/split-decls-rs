macro_rules! deps {
    () => {
        Mcg128Xsl64!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Debug for Mcg128Xsl64 { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Mcg128Xsl64 {{}}") } }
    };
}

impl_10!();