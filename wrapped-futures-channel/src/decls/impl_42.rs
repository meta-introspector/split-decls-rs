macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl fmt :: Display for SendError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_full () { write ! (f , "send failed because channel is full") } else { write ! (f , "send failed because receiver is gone") } } }
    };
}

impl_42!()