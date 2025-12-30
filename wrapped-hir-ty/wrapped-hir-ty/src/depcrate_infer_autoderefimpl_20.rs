// Generated macro for impl_20 (impl)
macro_rules! Depcrate_infer_autoderefimpl_20 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'db > TrackAutoderefSteps < 'db > for Vec < (Ty < 'db > , AutoderefKind) > { fn len (& self) -> usize { self . len () } fn push (& mut self , ty : Ty < 'db > , kind : AutoderefKind) { self . push ((ty , kind)) ; } }
};
}
