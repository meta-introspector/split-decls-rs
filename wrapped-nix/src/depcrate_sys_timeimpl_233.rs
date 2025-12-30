// Generated macro for impl_233 (impl)
macro_rules! Depcrate_sys_timeimpl_233 {
() => {
// Module: crate::sys::time
// Provides: {"impl_233"}
// Dependencies: {}
impl ops :: Add for TimeVal { type Output = TimeVal ; fn add (self , rhs : TimeVal) -> TimeVal { TimeVal :: microseconds (self . num_microseconds () + rhs . num_microseconds ()) } }
};
}
