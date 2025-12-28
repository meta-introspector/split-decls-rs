macro_rules! deps {
    () => {
        TimeZone!();
        TimeDelta!();
        Date!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < Tz : TimeZone > Sub < TimeDelta > for Date < Tz > { type Output = Date < Tz > ; # [inline] fn sub (self , rhs : TimeDelta) -> Date < Tz > { self . checked_sub_signed (rhs) . expect ("`Date - TimeDelta` overflowed") } }
    };
}

impl_50!();