macro_rules! deps {
    () => {
        PackageId!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Display for PackageId { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . repr , f) } }
    };
}

impl_9!()