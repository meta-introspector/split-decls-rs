macro_rules! deps {
    () => {
        PackageId!();
        Result!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl fmt :: Display for PackageId { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . repr , f) } }
    };
}

impl_44!()