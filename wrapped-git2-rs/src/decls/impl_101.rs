macro_rules! deps {
    () => {
        ObjectType!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl fmt :: Display for ObjectType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . str () . fmt (f) } }
    };
}

impl_101!()