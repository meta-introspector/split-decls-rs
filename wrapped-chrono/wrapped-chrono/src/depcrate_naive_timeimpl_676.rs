// Generated macro for impl_676 (impl)
macro_rules! Depcrate_naive_timeimpl_676 {
() => {
// Module: crate::naive::time
// Provides: {"impl_676"}
// Dependencies: {}
# [doc = " Add `FixedOffset` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl Add < FixedOffset > for NaiveTime { type Output = NaiveTime ; # [inline] fn add (self , rhs : FixedOffset) -> NaiveTime { self . overflowing_add_offset (rhs) . 0 } }
};
}
