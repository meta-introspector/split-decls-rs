// Generated macro for impl_215 (impl)
macro_rules! Depcrate_sys_timeimpl_215 {
() => {
// Module: crate::sys::time
// Provides: {"impl_215"}
// Dependencies: {}
impl ops :: Neg for TimeSpec { type Output = TimeSpec ; fn neg (self) -> TimeSpec { TimeSpec :: nanoseconds (- self . num_nanoseconds ()) } }
};
}
