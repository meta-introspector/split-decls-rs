// Generated macro for impl_675 (impl)
macro_rules! Depcrate_naive_timeimpl_675 {
() => {
// Module: crate::naive::time
// Provides: {"impl_675"}
// Dependencies: {}
# [doc = " Add-assign `std::time::Duration` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl AddAssign < Duration > for NaiveTime { # [inline] fn add_assign (& mut self , rhs : Duration) { * self = * self + rhs ; } }
};
}
