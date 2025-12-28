macro_rules! deps {
    () => {
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < Tz : TimeZone > fmt :: Debug for DateTime < Tz > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . overflowing_naive_local () . fmt (f) ? ; self . offset . fmt (f) } }
    };
}

impl_182!();