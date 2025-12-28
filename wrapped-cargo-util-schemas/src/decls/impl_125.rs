macro_rules! deps {
    () => {
        ProfilePackageSpec!();
        Result!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl fmt :: Display for ProfilePackageSpec { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ProfilePackageSpec :: Spec (spec) => spec . fmt (f) , ProfilePackageSpec :: All => f . write_str ("*") , } } }
    };
}

impl_125!()