macro_rules! deps {
    () => {
        TimeDelta!();
        Date!();
        TimeZone!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < Tz : TimeZone > Add < TimeDelta > for Date < Tz > { type Output = Date < Tz > ; # [inline] fn add (self , rhs : TimeDelta) -> Date < Tz > { self . checked_add_signed (rhs) . expect ("`Date + TimeDelta` overflowed") } }
    };
}

impl_48!()