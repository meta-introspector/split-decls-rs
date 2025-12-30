// Generated macro for impl_234 (impl)
macro_rules! Depcrate_sys_timeimpl_234 {
() => {
// Module: crate::sys::time
// Provides: {"impl_234"}
// Dependencies: {}
impl ops :: Sub for TimeVal { type Output = TimeVal ; fn sub (self , rhs : TimeVal) -> TimeVal { TimeVal :: microseconds (self . num_microseconds () - rhs . num_microseconds ()) } }
};
}
