// Generated macro for impl_29 (impl)
macro_rules! Depcrate_time_deltaimpl_29 {
() => {
// Module: crate::time_delta
// Provides: {"impl_29"}
// Dependencies: {}
impl Div < i32 > for TimeDelta { type Output = TimeDelta ; fn div (self , rhs : i32) -> TimeDelta { self . checked_div (rhs) . expect ("`i32` is zero") } }
};
}
