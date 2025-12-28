macro_rules! deps {
    () => {
        TimeZone!();
        TimeDelta!();
        NaiveDateTime!();
        DateTime!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        # [doc = " Add-assign `chrono::Duration` to `DateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_add_signed`] to get an `Option` instead."] impl < Tz : TimeZone > AddAssign < TimeDelta > for DateTime < Tz > { # [inline] fn add_assign (& mut self , rhs : TimeDelta) { let datetime = self . datetime . checked_add_signed (rhs) . expect ("`DateTime + TimeDelta` overflowed") ; let tz = self . timezone () ; * self = tz . from_utc_datetime (& datetime) ; } }
    };
}

impl_168!()