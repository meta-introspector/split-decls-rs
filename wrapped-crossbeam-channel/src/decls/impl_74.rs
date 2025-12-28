macro_rules! deps {
    () => {
        TrySendError!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T > fmt :: Debug for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Full (..) => "Full(..)" . fmt (f) , Self :: Disconnected (..) => "Disconnected(..)" . fmt (f) , } } }
    };
}

impl_74!();