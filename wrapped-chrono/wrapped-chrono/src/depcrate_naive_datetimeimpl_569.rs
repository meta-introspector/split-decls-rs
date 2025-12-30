// Generated macro for impl_569 (impl)
macro_rules! Depcrate_naive_datetimeimpl_569 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_569"}
// Dependencies: {}
# [doc = " Add `FixedOffset` to `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_add_offset` to get an `Option` instead."] impl Add < FixedOffset > for NaiveDateTime { type Output = NaiveDateTime ; # [inline] fn add (self , rhs : FixedOffset) -> NaiveDateTime { self . checked_add_offset (rhs) . expect ("`NaiveDateTime + FixedOffset` out of range") } }
};
}
