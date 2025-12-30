// Generated macro for impl_26 (impl)
macro_rules! Depcrate_time_deltaimpl_26 {
() => {
// Module: crate::time_delta
// Provides: {"impl_26"}
// Dependencies: {}
impl AddAssign for TimeDelta { fn add_assign (& mut self , rhs : TimeDelta) { let new = self . checked_add (& rhs) . expect ("`TimeDelta + TimeDelta` overflowed") ; * self = new ; } }
};
}
