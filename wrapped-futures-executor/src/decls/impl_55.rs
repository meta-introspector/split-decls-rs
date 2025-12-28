macro_rules! deps {
    () => {
        EnterError!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl fmt :: Debug for EnterError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("EnterError") . finish () } }
    };
}

impl_55!();