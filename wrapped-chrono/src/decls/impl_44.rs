macro_rules! deps {
    () => {
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < Tz : TimeZone > Eq for Date < Tz > { }
    };
}

impl_44!();