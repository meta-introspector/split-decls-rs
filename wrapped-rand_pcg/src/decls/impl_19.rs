macro_rules! deps {
    () => {
        Lcg128CmDxsm64!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Debug for Lcg128CmDxsm64 { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Lcg128CmDxsm64 {{}}") } }
    };
}

impl_19!()