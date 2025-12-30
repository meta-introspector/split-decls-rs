// Generated macro for impl_27 (impl)
macro_rules! Depcrate_time_deltaimpl_27 {
() => {
// Module: crate::time_delta
// Provides: {"impl_27"}
// Dependencies: {}
impl SubAssign for TimeDelta { fn sub_assign (& mut self , rhs : TimeDelta) { let new = self . checked_sub (& rhs) . expect ("`TimeDelta - TimeDelta` overflowed") ; * self = new ; } }
};
}
