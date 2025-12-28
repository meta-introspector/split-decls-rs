macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl fmt :: Debug for Id { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "Id({})" , self . id) } }
    };
}

impl_169!()