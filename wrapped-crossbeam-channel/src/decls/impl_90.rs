macro_rules! deps {
    () => {
        RecvTimeoutError!();
        Timeout!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl fmt :: Display for RecvTimeoutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Timeout => "timed out waiting on receive operation" . fmt (f) , Self :: Disconnected => "channel is empty and disconnected" . fmt (f) , } } }
    };
}

impl_90!()