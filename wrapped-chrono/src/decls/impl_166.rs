macro_rules! deps {
    () => {
        DateTime!();
        NaiveDateTime!();
        TimeDelta!();
        TimeZone!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        # [doc = " Add `TimeDelta` to `DateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_add_signed`] to get an `Option` instead."] impl < Tz : TimeZone > Add < TimeDelta > for DateTime < Tz > { type Output = DateTime < Tz > ; # [inline] fn add (self , rhs : TimeDelta) -> DateTime < Tz > { self . checked_add_signed (rhs) . expect ("`DateTime + TimeDelta` overflowed") } }
    };
}

impl_166!();