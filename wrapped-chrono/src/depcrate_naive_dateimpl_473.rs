// Generated macro for impl_473 (impl)
macro_rules! Depcrate_naive_dateimpl_473 {
() => {
// Module: crate::naive::date
// Provides: {"impl_473"}
// Dependencies: {}
# [doc = " Add `Days` to `NaiveDate`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `NaiveDate::checked_add_days` to get an `Option` instead."] impl Add < Days > for NaiveDate { type Output = NaiveDate ; fn add (self , days : Days) -> Self :: Output { self . checked_add_days (days) . expect ("`NaiveDate + Days` out of range") } }
};
}
