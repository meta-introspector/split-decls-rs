macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl fmt :: Display for Utf8Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "invalid UTF-8 found at byte offset {}" , self . valid_up_to) } }
    };
}

impl_238!();