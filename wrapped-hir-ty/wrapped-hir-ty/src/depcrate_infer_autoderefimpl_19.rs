// Generated macro for impl_19 (impl)
macro_rules! Depcrate_infer_autoderefimpl_19 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'db > TrackAutoderefSteps < 'db > for usize { fn len (& self) -> usize { * self } fn push (& mut self , _ : Ty < 'db > , _ : AutoderefKind) { * self += 1 ; } }
};
}
