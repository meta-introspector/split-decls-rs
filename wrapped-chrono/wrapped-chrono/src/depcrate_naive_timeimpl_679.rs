// Generated macro for impl_679 (impl)
macro_rules! Depcrate_naive_timeimpl_679 {
() => {
// Module: crate::naive::time
// Provides: {"impl_679"}
// Dependencies: {}
# [doc = " Subtract `std::time::Duration` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl Sub < Duration > for NaiveTime { type Output = NaiveTime ; # [inline] fn sub (self , rhs : Duration) -> NaiveTime { let secs = rhs . as_secs () % (2 * 24 * 60 * 60) ; let d = TimeDelta :: new (secs as i64 , rhs . subsec_nanos ()) . unwrap () ; self . overflowing_sub_signed (d) . 0 } }
};
}
