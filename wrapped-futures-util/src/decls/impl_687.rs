macro_rules! deps {
    () => {
        TryReadyChunksError!();
    };
}

macro_rules! impl_687 {
    () => {
        deps!();
        impl < T , E : fmt :: Display > fmt :: Display for TryReadyChunksError < T , E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 1 . fmt (f) } }
    };
}

impl_687!()