// Generated macro for impl_474 (impl)
macro_rules! Depcrate_naive_dateimpl_474 {
() => {
// Module: crate::naive::date
// Provides: {"impl_474"}
// Dependencies: {}
# [doc = " Subtract `Days` from `NaiveDate`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `NaiveDate::checked_sub_days` to get an `Option` instead."] impl Sub < Days > for NaiveDate { type Output = NaiveDate ; fn sub (self , days : Days) -> Self :: Output { self . checked_sub_days (days) . expect ("`NaiveDate - Days` out of range") } }
};
}
