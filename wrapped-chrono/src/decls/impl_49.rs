macro_rules! deps {
    () => {
        TimeDelta!();
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < Tz : TimeZone > AddAssign < TimeDelta > for Date < Tz > { # [inline] fn add_assign (& mut self , rhs : TimeDelta) { self . date = self . date . checked_add_signed (rhs) . expect ("`Date + TimeDelta` overflowed") ; } }
    };
}

impl_49!();