macro_rules! deps {
    () => {
        Any!();
        Validate!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl < T : Validate > Debug for Any < T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { self . logical_name () . fmt (f) } }
    };
}

impl_762!()