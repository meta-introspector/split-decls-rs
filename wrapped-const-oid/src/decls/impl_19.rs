macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > fmt :: Debug for ObjectIdentifier < MAX_SIZE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ObjectIdentifier({self})") } }
    };
}

impl_19!()