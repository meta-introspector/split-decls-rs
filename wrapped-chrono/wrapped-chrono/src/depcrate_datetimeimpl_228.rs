// Generated macro for impl_228 (impl)
macro_rules! Depcrate_datetimeimpl_228 {
() => {
// Module: crate::datetime
// Provides: {"impl_228"}
// Dependencies: {}
# [doc = " Subtract-assign `std::time::Duration` from `DateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `DateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_sub_signed`] to get an `Option` instead."] impl < Tz : TimeZone > SubAssign < Duration > for DateTime < Tz > { # [inline] fn sub_assign (& mut self , rhs : Duration) { let rhs = TimeDelta :: from_std (rhs) . expect ("overflow converting from core::time::Duration to TimeDelta") ; * self -= rhs ; } }
};
}
