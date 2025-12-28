macro_rules! deps {
    () => {
        NaiveDateTime!();
        Duration!();
        TimeZone!();
        TimeDelta!();
        DateTime!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        # [doc = " Add-assign `std::time::Duration` to `DateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_add_signed`] to get an `Option` instead."] impl < Tz : TimeZone > AddAssign < Duration > for DateTime < Tz > { # [inline] fn add_assign (& mut self , rhs : Duration) { let rhs = TimeDelta :: from_std (rhs) . expect ("overflow converting from core::time::Duration to TimeDelta") ; * self += rhs ; } }
    };
}

impl_169!()