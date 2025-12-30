// Generated macro for impl_578 (impl)
macro_rules! Depcrate_naive_datetimeimpl_578 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_578"}
// Dependencies: {}
# [doc = " Add `Days` to `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_add_days` to get an `Option` instead."] impl Add < Days > for NaiveDateTime { type Output = NaiveDateTime ; fn add (self , days : Days) -> Self :: Output { self . checked_add_days (days) . expect ("`NaiveDateTime + Days` out of range") } }
};
}
