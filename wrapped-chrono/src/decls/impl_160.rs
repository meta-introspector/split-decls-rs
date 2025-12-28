macro_rules! deps {
    () => {
        NaiveDateTime!();
        DateTime!();
        Offset!();
        TimeZone!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < Tz : TimeZone > Copy for DateTime < Tz > where < Tz as TimeZone > :: Offset : Copy , NaiveDateTime : Copy , { }
    };
}

impl_160!();