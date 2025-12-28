macro_rules! deps {
    () => {
        Demangle!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Demangle < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
    };
}

impl_48!()