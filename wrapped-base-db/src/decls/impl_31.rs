macro_rules! deps {
    () => {
        CrateDisplayName!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl fmt :: Display for CrateDisplayName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . crate_name . fmt (f) } }
    };
}

impl_31!();