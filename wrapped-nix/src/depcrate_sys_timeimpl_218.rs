// Generated macro for impl_218 (impl)
macro_rules! Depcrate_sys_timeimpl_218 {
() => {
// Module: crate::sys::time
// Provides: {"impl_218"}
// Dependencies: {}
impl ops :: Mul < i32 > for TimeSpec { type Output = TimeSpec ; fn mul (self , rhs : i32) -> TimeSpec { let usec = self . num_nanoseconds () . checked_mul (i64 :: from (rhs)) . expect ("TimeSpec multiply out of bounds") ; TimeSpec :: nanoseconds (usec) } }
};
}
