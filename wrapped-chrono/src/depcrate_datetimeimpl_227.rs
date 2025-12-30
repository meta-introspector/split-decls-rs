// Generated macro for impl_227 (impl)
macro_rules! Depcrate_datetimeimpl_227 {
() => {
// Module: crate::datetime
// Provides: {"impl_227"}
// Dependencies: {}
# [doc = " Subtract-assign `TimeDelta` from `DateTime`."] # [doc = ""] # [doc = " This is the same as the addition with a negated `TimeDelta`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `DateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`DateTime<Tz>::checked_sub_signed`] to get an `Option` instead."] impl < Tz : TimeZone > SubAssign < TimeDelta > for DateTime < Tz > { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { let datetime = self . datetime . checked_sub_signed (rhs) . expect ("`DateTime - TimeDelta` overflowed") ; let tz = self . timezone () ; * self = tz . from_utc_datetime (& datetime) } }
};
}
