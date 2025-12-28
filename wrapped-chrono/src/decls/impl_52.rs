macro_rules! deps {
    () => {
        TimeDelta!();
        Date!();
        TimeZone!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < Tz : TimeZone > Sub < Date < Tz > > for Date < Tz > { type Output = TimeDelta ; # [inline] fn sub (self , rhs : Date < Tz >) -> TimeDelta { self . signed_duration_since (rhs) } }
    };
}

impl_52!()