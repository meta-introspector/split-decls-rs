// Generated macro for impl_119 (impl)
macro_rules! Depcrate_constraintsimpl_119 {
() => {
// Module: crate::constraints
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'tcx > Index < OutlivesConstraintIndex > for OutlivesConstraintSet < 'tcx > { type Output = OutlivesConstraint < 'tcx > ; fn index (& self , i : OutlivesConstraintIndex) -> & Self :: Output { & self . outlives [i] } }
};
}
