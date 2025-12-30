// Generated macro for impl_681 (impl)
macro_rules! Depcrate_naive_timeimpl_681 {
() => {
// Module: crate::naive::time
// Provides: {"impl_681"}
// Dependencies: {}
# [doc = " Subtract `FixedOffset` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl Sub < FixedOffset > for NaiveTime { type Output = NaiveTime ; # [inline] fn sub (self , rhs : FixedOffset) -> NaiveTime { self . overflowing_sub_offset (rhs) . 0 } }
};
}
