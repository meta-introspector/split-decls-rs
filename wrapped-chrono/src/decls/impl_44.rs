macro_rules! deps {
    () => {
        Date!();
        TimeZone!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < Tz : TimeZone > Eq for Date < Tz > { }
    };
}

impl_44!()