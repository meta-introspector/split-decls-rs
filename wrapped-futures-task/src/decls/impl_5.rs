macro_rules! deps {
    () => {
        SpawnError!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: Debug for SpawnError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("SpawnError") . field (& "shutdown") . finish () } }
    };
}

impl_5!();