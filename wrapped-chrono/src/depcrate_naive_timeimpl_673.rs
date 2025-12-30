// Generated macro for impl_673 (impl)
macro_rules! Depcrate_naive_timeimpl_673 {
() => {
// Module: crate::naive::time
// Provides: {"impl_673"}
// Dependencies: {}
# [doc = " Add-assign `TimeDelta` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl AddAssign < TimeDelta > for NaiveTime { # [inline] fn add_assign (& mut self , rhs : TimeDelta) { * self = self . add (rhs) ; } }
};
}
