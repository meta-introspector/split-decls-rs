// Generated macro for impl_28 (impl)
macro_rules! Depcrate_time_deltaimpl_28 {
() => {
// Module: crate::time_delta
// Provides: {"impl_28"}
// Dependencies: {}
impl Mul < i32 > for TimeDelta { type Output = TimeDelta ; fn mul (self , rhs : i32) -> TimeDelta { self . checked_mul (rhs) . expect ("`TimeDelta * i32` overflowed") } }
};
}
