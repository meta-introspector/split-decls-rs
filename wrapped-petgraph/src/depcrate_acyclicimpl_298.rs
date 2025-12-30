// Generated macro for impl_298 (impl)
macro_rules! Depcrate_acyclicimpl_298 {
() => {
// Module: crate::acyclic
// Provides: {"impl_298"}
// Dependencies: {}
impl < N > From < Cycle < N > > for AcyclicEdgeError < N > { fn from (cycle : Cycle < N >) -> Self { AcyclicEdgeError :: Cycle (cycle) } }
};
}
