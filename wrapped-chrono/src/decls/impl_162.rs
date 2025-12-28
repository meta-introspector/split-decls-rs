macro_rules! deps {
    () => {
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < Tz : TimeZone > Eq for DateTime < Tz > { }
    };
}

impl_162!();