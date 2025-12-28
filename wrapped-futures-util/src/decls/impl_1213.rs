macro_rules! deps {
    () => {
        ReuniteError!();
        ReadHalf!();
        WriteHalf!();
    };
}

macro_rules! impl_1213 {
    () => {
        deps!();
        impl < T > fmt :: Display for ReuniteError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "tried to reunite a ReadHalf and WriteHalf that don't form a pair") } }
    };
}

impl_1213!()