// Generated macro for impl_678 (impl)
macro_rules! Depcrate_naive_timeimpl_678 {
() => {
// Module: crate::naive::time
// Provides: {"impl_678"}
// Dependencies: {}
# [doc = " Subtract-assign `TimeDelta` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl SubAssign < TimeDelta > for NaiveTime { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { * self = self . sub (rhs) ; } }
};
}
