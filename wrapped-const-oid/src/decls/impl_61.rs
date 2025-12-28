macro_rules! deps {
    () => {
        Result!();
        ObjectIdentifier!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > fmt :: Debug for ObjectIdentifier < MAX_SIZE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ObjectIdentifier({self})") } }
    };
}

impl_61!();