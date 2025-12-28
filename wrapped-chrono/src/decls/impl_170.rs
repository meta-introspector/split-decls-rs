macro_rules! deps {
    () => {
        DateTime!();
        TimeZone!();
        FixedOffset!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        # [doc = " Add `FixedOffset` to the datetime value of `DateTime` (offset remains unchanged)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] impl < Tz : TimeZone > Add < FixedOffset > for DateTime < Tz > { type Output = DateTime < Tz > ; # [inline] fn add (mut self , rhs : FixedOffset) -> DateTime < Tz > { self . datetime = self . naive_utc () . checked_add_offset (rhs) . expect ("`DateTime + FixedOffset` overflowed") ; self } }
    };
}

impl_170!()