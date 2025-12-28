macro_rules! deps {
    () => {
        Result!();
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl fmt :: Debug for ObjectIdentifierRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ObjectIdentifierRef({self})") } }
    };
}

impl_69!()