macro_rules! deps {
    () => {
        SendTimeoutError!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T > fmt :: Debug for SendTimeoutError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "SendTimeoutError(..)" . fmt (f) } }
    };
}

impl_79!();