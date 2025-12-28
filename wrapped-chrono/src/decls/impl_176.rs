macro_rules! deps {
    () => {
        TimeZone!();
        FixedOffset!();
        DateTime!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        # [doc = " Subtract `FixedOffset` from the datetime value of `DateTime` (offset remains unchanged)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] impl < Tz : TimeZone > Sub < FixedOffset > for DateTime < Tz > { type Output = DateTime < Tz > ; # [inline] fn sub (mut self , rhs : FixedOffset) -> DateTime < Tz > { self . datetime = self . naive_utc () . checked_sub_offset (rhs) . expect ("`DateTime - FixedOffset` overflowed") ; self } }
    };
}

impl_176!();