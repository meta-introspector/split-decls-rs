macro_rules! deps {
    () => {
        TimeZone!();
        DateTime!();
        TimeDelta!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < Tz : TimeZone > Sub < & DateTime < Tz > > for DateTime < Tz > { type Output = TimeDelta ; # [inline] fn sub (self , rhs : & DateTime < Tz >) -> TimeDelta { self . signed_duration_since (rhs) } }
    };
}

impl_179!();