macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T > fmt :: Display for SendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "sending on a disconnected channel" . fmt (f) } }
    };
}

impl_71!()