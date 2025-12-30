// Generated macro for impl_222 (impl)
macro_rules! Depcrate_datetimeimpl_222 {
() => {
// Module: crate::datetime
// Provides: {"impl_222"}
// Dependencies: {}
# [doc = " Add-assign `std::time::Duration` to `DateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_add_signed`] to get an `Option` instead."] impl < Tz : TimeZone > AddAssign < Duration > for DateTime < Tz > { # [inline] fn add_assign (& mut self , rhs : Duration) { let rhs = TimeDelta :: from_std (rhs) . expect ("overflow converting from core::time::Duration to TimeDelta") ; * self += rhs ; } }
};
}
