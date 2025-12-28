macro_rules! deps {
    () => {
        TimeDelta!();
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        # [doc = " Subtract `TimeDelta` from `DateTime`."] # [doc = ""] # [doc = " This is the same as the addition with a negated `TimeDelta`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling] the subtraction assumes that **there is no leap"] # [doc = " second ever**, except when the `DateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_sub_signed`] to get an `Option` instead."] impl < Tz : TimeZone > Sub < TimeDelta > for DateTime < Tz > { type Output = DateTime < Tz > ; # [inline] fn sub (self , rhs : TimeDelta) -> DateTime < Tz > { self . checked_sub_signed (rhs) . expect ("`DateTime - TimeDelta` overflowed") } }
    };
}

impl_172!()