macro_rules! deps {
    () => {
        RecvError!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl fmt :: Display for RecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "receiving on an empty and disconnected channel" . fmt (f) } }
    };
}

impl_84!();