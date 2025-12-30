// Generated macro for impl_216 (impl)
macro_rules! Depcrate_sys_timeimpl_216 {
() => {
// Module: crate::sys::time
// Provides: {"impl_216"}
// Dependencies: {}
impl ops :: Add for TimeSpec { type Output = TimeSpec ; fn add (self , rhs : TimeSpec) -> TimeSpec { TimeSpec :: nanoseconds (self . num_nanoseconds () + rhs . num_nanoseconds ()) } }
};
}
