macro_rules! deps {
    () => {
        TrySendError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T > fmt :: Display for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_full () { write ! (f , "send failed because channel is full") } else { write ! (f , "send failed because receiver is gone") } } }
    };
}

impl_49!();