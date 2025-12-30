// Generated macro for impl_377 (impl)
macro_rules! Depcrate_autoderefimpl_377 {
() => {
// Module: crate::autoderef
// Provides: {"impl_377"}
// Dependencies: {}
# [allow (private_bounds)] impl < T : TrackAutoderefSteps > Autoderef < '_ , '_ , T > { pub (crate) fn step_count (& self) -> usize { self . steps . len () } pub (crate) fn final_ty (& self) -> Ty { self . ty . clone () } }
};
}
