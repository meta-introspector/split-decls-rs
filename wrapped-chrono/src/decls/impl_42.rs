macro_rules! deps {
    () => {
        Offset!();
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        unsafe impl < Tz : TimeZone > Send for Date < Tz > where < Tz as TimeZone > :: Offset : Send { }
    };
}

impl_42!()