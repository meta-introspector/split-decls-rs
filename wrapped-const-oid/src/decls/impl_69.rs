macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        Result!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl fmt :: Debug for ObjectIdentifierRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ObjectIdentifierRef({self})") } }
    };
}

impl_69!();