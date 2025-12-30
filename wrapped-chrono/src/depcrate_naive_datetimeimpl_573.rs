// Generated macro for impl_573 (impl)
macro_rules! Depcrate_naive_datetimeimpl_573 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_573"}
// Dependencies: {}
# [doc = " Subtract-assign `TimeDelta` from `NaiveDateTime`."] # [doc = ""] # [doc = " This is the same as the addition with a negated `TimeDelta`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap  second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_sub_signed`] to get an `Option` instead."] impl SubAssign < TimeDelta > for NaiveDateTime { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { * self = self . sub (rhs) ; } }
};
}
