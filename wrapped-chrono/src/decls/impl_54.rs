macro_rules! deps {
    () => {
        Offset!();
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < Tz : TimeZone > fmt :: Display for Date < Tz > where Tz :: Offset : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . naive_local () . fmt (f) ? ; self . offset . fmt (f) } }
    };
}

impl_54!();