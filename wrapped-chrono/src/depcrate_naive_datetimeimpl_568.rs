// Generated macro for impl_568 (impl)
macro_rules! Depcrate_naive_datetimeimpl_568 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_568"}
// Dependencies: {}
# [doc = " Add-assign `std::time::Duration` to `NaiveDateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap  second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_add_signed`] to get an `Option` instead."] impl AddAssign < Duration > for NaiveDateTime { # [inline] fn add_assign (& mut self , rhs : Duration) { * self = self . add (rhs) ; } }
};
}
