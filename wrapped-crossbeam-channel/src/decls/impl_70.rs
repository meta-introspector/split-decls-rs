macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T > fmt :: Debug for SendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "SendError(..)" . fmt (f) } }
    };
}

impl_70!()