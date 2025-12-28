macro_rules! deps {
    () => {
        ReuniteError!();
        SplitStream!();
        SplitSink!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < T , Item > fmt :: Display for ReuniteError < T , Item > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "tried to reunite a SplitStream and SplitSink that don't form a pair") } }
    };
}

impl_568!();