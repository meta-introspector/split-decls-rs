macro_rules! deps {
    () => {
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < Tz : TimeZone > fmt :: Debug for Date < Tz > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . naive_local () . fmt (f) ? ; self . offset . fmt (f) } }
    };
}

impl_53!();