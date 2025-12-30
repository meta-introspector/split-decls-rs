// Generated macro for impl_217 (impl)
macro_rules! Depcrate_sys_timeimpl_217 {
() => {
// Module: crate::sys::time
// Provides: {"impl_217"}
// Dependencies: {}
impl ops :: Sub for TimeSpec { type Output = TimeSpec ; fn sub (self , rhs : TimeSpec) -> TimeSpec { TimeSpec :: nanoseconds (self . num_nanoseconds () - rhs . num_nanoseconds ()) } }
};
}
