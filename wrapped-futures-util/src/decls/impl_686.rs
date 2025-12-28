macro_rules! deps {
    () => {
        TryReadyChunksError!();
    };
}

macro_rules! impl_686 {
    () => {
        deps!();
        impl < T , E : fmt :: Debug > fmt :: Debug for TryReadyChunksError < T , E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 1 . fmt (f) } }
    };
}

impl_686!();