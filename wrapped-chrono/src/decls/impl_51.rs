macro_rules! deps {
    () => {
        TimeZone!();
        Date!();
        TimeDelta!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < Tz : TimeZone > SubAssign < TimeDelta > for Date < Tz > { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { self . date = self . date . checked_sub_signed (rhs) . expect ("`Date - TimeDelta` overflowed") ; } }
    };
}

impl_51!()