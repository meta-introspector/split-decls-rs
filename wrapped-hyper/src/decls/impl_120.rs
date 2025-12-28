macro_rules! deps {
    () => {
        Result!();
        TimedOut!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl fmt :: Display for TimedOut { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("operation timed out") } }
    };
}

impl_120!();