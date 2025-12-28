macro_rules! deps {
    () => {
        Validate!();
        Any!();
    };
}

macro_rules! impl_763 {
    () => {
        deps!();
        impl < T : Validate > std :: fmt :: Display for Any < T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (& self . logical_name ()) } }
    };
}

impl_763!()