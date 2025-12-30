// Generated macro for impl_575 (impl)
macro_rules! Depcrate_naive_datetimeimpl_575 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_575"}
// Dependencies: {}
# [doc = " Subtract `FixedOffset` from `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_sub_offset` to get an `Option` instead."] impl Sub < FixedOffset > for NaiveDateTime { type Output = NaiveDateTime ; # [inline] fn sub (self , rhs : FixedOffset) -> NaiveDateTime { self . checked_sub_offset (rhs) . expect ("`NaiveDateTime - FixedOffset` out of range") } }
};
}
