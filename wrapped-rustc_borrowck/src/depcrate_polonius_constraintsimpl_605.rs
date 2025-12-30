// Generated macro for impl_605 (impl)
macro_rules! Depcrate_polonius_constraintsimpl_605 {
() => {
// Module: crate::polonius::constraints
// Provides: {"impl_605"}
// Dependencies: {}
impl LocalizedOutlivesConstraintSet { pub (crate) fn push (& mut self , constraint : LocalizedOutlivesConstraint) { if constraint . source == constraint . target && constraint . from == constraint . to { return ; } self . outlives . push (constraint) ; } }
};
}
