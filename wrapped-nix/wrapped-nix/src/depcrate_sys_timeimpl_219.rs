// Generated macro for impl_219 (impl)
macro_rules! Depcrate_sys_timeimpl_219 {
() => {
// Module: crate::sys::time
// Provides: {"impl_219"}
// Dependencies: {}
impl ops :: Div < i32 > for TimeSpec { type Output = TimeSpec ; fn div (self , rhs : i32) -> TimeSpec { let usec = self . num_nanoseconds () / i64 :: from (rhs) ; TimeSpec :: nanoseconds (usec) } }
};
}
