macro_rules! deps {
    () => {
        RecvError!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl fmt :: Display for RecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "receive failed because channel is empty and closed") } }
    };
}

impl_44!();