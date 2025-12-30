// Generated macro for impl_476 (impl)
macro_rules! Depcrate_naive_dateimpl_476 {
() => {
// Module: crate::naive::date
// Provides: {"impl_476"}
// Dependencies: {}
# [doc = " Subtract-assign `TimeDelta` from `NaiveDate`."] # [doc = ""] # [doc = " This discards the fractional days in `TimeDelta`, rounding to the closest integral number of"] # [doc = " days towards `TimeDelta::zero()`."] # [doc = " It is the same as the addition with a negated `TimeDelta`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDate::checked_sub_signed`] to get an `Option` instead."] impl SubAssign < TimeDelta > for NaiveDate { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { * self = self . sub (rhs) ; } }
};
}
