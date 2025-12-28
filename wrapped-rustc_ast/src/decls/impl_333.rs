macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl fmt :: Display for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as fmt :: Debug > :: fmt (self , f) } }
    };
}

impl_333!()