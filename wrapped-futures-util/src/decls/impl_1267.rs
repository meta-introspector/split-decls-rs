macro_rules! deps {
    () => {
        ReuniteError!();
    };
}

macro_rules! impl_1267 {
    () => {
        deps!();
        impl < T > fmt :: Display for ReuniteError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "tried to reunite two BiLocks that don't form a pair") } }
    };
}

impl_1267!()