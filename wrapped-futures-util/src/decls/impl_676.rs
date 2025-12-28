macro_rules! deps {
    () => {
        TryChunksError!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl < T , E : fmt :: Display > fmt :: Display for TryChunksError < T , E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 1 . fmt (f) } }
    };
}

impl_676!()