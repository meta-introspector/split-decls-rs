macro_rules! deps {
    () => {
        Offset!();
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < Tz : TimeZone > fmt :: Display for DateTime < Tz > where Tz :: Offset : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . overflowing_naive_local () . fmt (f) ? ; f . write_char (' ') ? ; self . offset . fmt (f) } }
    };
}

impl_185!()