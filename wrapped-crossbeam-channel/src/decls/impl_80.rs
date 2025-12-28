macro_rules! deps {
    () => {
        SendTimeoutError!();
        Timeout!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T > fmt :: Display for SendTimeoutError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Timeout (..) => "timed out waiting on send operation" . fmt (f) , Self :: Disconnected (..) => "sending on a disconnected channel" . fmt (f) , } } }
    };
}

impl_80!();