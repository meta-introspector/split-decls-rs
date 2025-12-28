macro_rules! deps {
    () => {
        Idx!();
        IndexSlice!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < I : Idx , T : fmt :: Debug > fmt :: Debug for IndexSlice < I , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . raw , fmt) } }
    };
}

impl_97!()