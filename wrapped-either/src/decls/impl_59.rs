macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < L , R > fmt :: Display for Either < L , R > where L : fmt :: Display , R : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for_both ! (self , inner => inner . fmt (f)) } }
    };
}

impl_59!();