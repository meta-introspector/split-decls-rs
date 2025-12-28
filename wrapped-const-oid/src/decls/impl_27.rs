macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Debug for ObjectIdentifierRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ObjectIdentifierRef({self})") } }
    };
}

impl_27!()