macro_rules! deps {
    () => {
        DateTime!();
        TimeZone!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < Tz : TimeZone > Eq for DateTime < Tz > { }
    };
}

impl_162!()