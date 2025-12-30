// Generated macro for impl_232 (impl)
macro_rules! Depcrate_sys_timeimpl_232 {
() => {
// Module: crate::sys::time
// Provides: {"impl_232"}
// Dependencies: {}
impl ops :: Neg for TimeVal { type Output = TimeVal ; fn neg (self) -> TimeVal { TimeVal :: microseconds (- self . num_microseconds ()) } }
};
}
