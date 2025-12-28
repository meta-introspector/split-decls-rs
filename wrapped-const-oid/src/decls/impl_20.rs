macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > fmt :: Display for ObjectIdentifier < MAX_SIZE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . as_oid_ref ()) } }
    };
}

impl_20!()