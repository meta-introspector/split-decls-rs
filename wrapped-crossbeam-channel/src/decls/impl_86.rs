macro_rules! deps {
    () => {
        TryRecvError!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl fmt :: Display for TryRecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Empty => "receiving on an empty channel" . fmt (f) , Self :: Disconnected => "receiving on an empty and disconnected channel" . fmt (f) , } } }
    };
}

impl_86!()