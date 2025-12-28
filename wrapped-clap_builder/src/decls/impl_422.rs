macro_rules! deps {
    () => {
        Result!();
        Backtrace!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        # [cfg (feature = "debug")] impl Display for Backtrace { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}" , self . 0) } }
    };
}

impl_422!();