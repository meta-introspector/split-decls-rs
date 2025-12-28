macro_rules! deps {
    () => {
        NaiveDateTime!();
        Duration!();
        TimeZone!();
        DateTime!();
        TimeDelta!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        # [doc = " Add `std::time::Duration` to `DateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_add_signed`] to get an `Option` instead."] impl < Tz : TimeZone > Add < Duration > for DateTime < Tz > { type Output = DateTime < Tz > ; # [inline] fn add (self , rhs : Duration) -> DateTime < Tz > { let rhs = TimeDelta :: from_std (rhs) . expect ("overflow converting from core::time::Duration to TimeDelta") ; self . checked_add_signed (rhs) . expect ("`DateTime + TimeDelta` overflowed") } }
    };
}

impl_167!();