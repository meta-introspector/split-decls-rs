// Generated macro for impl_232 (impl)
macro_rules! Depcrate_positionimpl_232 {
() => {
// Module: crate::position
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'i > Ord for Position < 'i > { fn cmp (& self , other : & Position < 'i >) -> Ordering { self . partial_cmp (other) . expect ("cannot compare positions from different strs") } }
};
}
