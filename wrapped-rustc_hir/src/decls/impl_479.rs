macro_rules! deps {
    () => {
        RustcVersion!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl Display for RustcVersion { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "{}.{}.{}" , self . major , self . minor , self . patch) } }
    };
}

impl_479!()