// Generated macro for impl_579 (impl)
macro_rules! Depcrate_naive_datetimeimpl_579 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_579"}
// Dependencies: {}
# [doc = " Subtract `Days` from `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_sub_days` to get an `Option` instead."] impl Sub < Days > for NaiveDateTime { type Output = NaiveDateTime ; fn sub (self , days : Days) -> Self :: Output { self . checked_sub_days (days) . expect ("`NaiveDateTime - Days` out of range") } }
};
}
