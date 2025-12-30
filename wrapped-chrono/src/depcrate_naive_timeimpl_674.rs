// Generated macro for impl_674 (impl)
macro_rules! Depcrate_naive_timeimpl_674 {
() => {
// Module: crate::naive::time
// Provides: {"impl_674"}
// Dependencies: {}
# [doc = " Add `std::time::Duration` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl Add < Duration > for NaiveTime { type Output = NaiveTime ; # [inline] fn add (self , rhs : Duration) -> NaiveTime { let secs = rhs . as_secs () % (2 * 24 * 60 * 60) ; let d = TimeDelta :: new (secs as i64 , rhs . subsec_nanos ()) . unwrap () ; self . overflowing_add_signed (d) . 0 } }
};
}
