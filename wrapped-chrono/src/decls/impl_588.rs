macro_rules! deps {
    () => {
        TimeZoneName!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl fmt :: Debug for TimeZoneName { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . as_ref () . fmt (f) } }
    };
}

impl_588!()