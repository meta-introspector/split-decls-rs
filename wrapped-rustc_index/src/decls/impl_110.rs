macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < I : Idx , T : fmt :: Debug > fmt :: Debug for IndexVec < I , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . raw , fmt) } }
    };
}

impl_110!()