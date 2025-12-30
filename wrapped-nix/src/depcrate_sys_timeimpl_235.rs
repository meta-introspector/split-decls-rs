// Generated macro for impl_235 (impl)
macro_rules! Depcrate_sys_timeimpl_235 {
() => {
// Module: crate::sys::time
// Provides: {"impl_235"}
// Dependencies: {}
impl ops :: Mul < i32 > for TimeVal { type Output = TimeVal ; fn mul (self , rhs : i32) -> TimeVal { let usec = self . num_microseconds () . checked_mul (i64 :: from (rhs)) . expect ("TimeVal multiply out of bounds") ; TimeVal :: microseconds (usec) } }
};
}
