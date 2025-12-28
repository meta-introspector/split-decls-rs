macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl fmt :: Debug for ByteSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} ({} bytes)" , self , self . 0) } }
    };
}

impl_56!()