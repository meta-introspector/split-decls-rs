macro_rules! deps {
    () => {
        FromUtf8Error!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl fmt :: Display for FromUtf8Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . err) } }
    };
}

impl_127!();