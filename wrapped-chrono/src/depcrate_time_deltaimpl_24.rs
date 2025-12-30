// Generated macro for impl_24 (impl)
macro_rules! Depcrate_time_deltaimpl_24 {
() => {
// Module: crate::time_delta
// Provides: {"impl_24"}
// Dependencies: {}
impl Add for TimeDelta { type Output = TimeDelta ; fn add (self , rhs : TimeDelta) -> TimeDelta { self . checked_add (& rhs) . expect ("`TimeDelta + TimeDelta` overflowed") } }
};
}
