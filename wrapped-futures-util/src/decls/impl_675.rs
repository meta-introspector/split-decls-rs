macro_rules! deps {
    () => {
        TryChunksError!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl < T , E : fmt :: Debug > fmt :: Debug for TryChunksError < T , E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 1 . fmt (f) } }
    };
}

impl_675!();