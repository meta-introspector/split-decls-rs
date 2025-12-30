// Generated macro for impl_680 (impl)
macro_rules! Depcrate_naive_timeimpl_680 {
() => {
// Module: crate::naive::time
// Provides: {"impl_680"}
// Dependencies: {}
# [doc = " Subtract-assign `std::time::Duration` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl SubAssign < Duration > for NaiveTime { # [inline] fn sub_assign (& mut self , rhs : Duration) { * self = * self - rhs ; } }
};
}
