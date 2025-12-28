macro_rules! deps {
    () => {
        TrySendError!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T > fmt :: Display for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Full (..) => "sending on a full channel" . fmt (f) , Self :: Disconnected (..) => "sending on a disconnected channel" . fmt (f) , } } }
    };
}

impl_75!();