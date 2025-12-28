macro_rules! deps {
    () => {
        TryRecvError!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl fmt :: Display for TryRecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TryRecvError :: Empty => write ! (f , "receive failed because channel is empty") , TryRecvError :: Closed => write ! (f , "receive failed because channel is closed") , } } }
    };
}

impl_52!();