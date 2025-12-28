macro_rules! deps {
    () => {
        TimedOut!();
        Result!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Display for TimedOut { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("operation timed out") } }
    };
}

impl_28!();