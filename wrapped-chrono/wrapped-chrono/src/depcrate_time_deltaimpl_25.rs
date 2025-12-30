// Generated macro for impl_25 (impl)
macro_rules! Depcrate_time_deltaimpl_25 {
() => {
// Module: crate::time_delta
// Provides: {"impl_25"}
// Dependencies: {}
impl Sub for TimeDelta { type Output = TimeDelta ; fn sub (self , rhs : TimeDelta) -> TimeDelta { self . checked_sub (& rhs) . expect ("`TimeDelta - TimeDelta` overflowed") } }
};
}
