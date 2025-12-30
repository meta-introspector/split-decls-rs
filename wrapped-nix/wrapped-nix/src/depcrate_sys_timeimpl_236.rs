// Generated macro for impl_236 (impl)
macro_rules! Depcrate_sys_timeimpl_236 {
() => {
// Module: crate::sys::time
// Provides: {"impl_236"}
// Dependencies: {}
impl ops :: Div < i32 > for TimeVal { type Output = TimeVal ; fn div (self , rhs : i32) -> TimeVal { let usec = self . num_microseconds () / i64 :: from (rhs) ; TimeVal :: microseconds (usec) } }
};
}
