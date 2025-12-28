macro_rules! deps {
    () => {
        Result!();
        Time!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl fmt :: Debug for Time { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Time") . finish () } }
    };
}

impl_86!();