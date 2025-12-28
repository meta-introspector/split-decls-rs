macro_rules! deps {
    () => {
        Lcg128Xsl64!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Debug for Lcg128Xsl64 { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Lcg128Xsl64 {{}}") } }
    };
}

impl_4!()