macro_rules! deps {
    () => {
        Date!();
        TimeZone!();
        Offset!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < Tz : TimeZone > Copy for Date < Tz > where < Tz as TimeZone > :: Offset : Copy { }
    };
}

impl_41!();