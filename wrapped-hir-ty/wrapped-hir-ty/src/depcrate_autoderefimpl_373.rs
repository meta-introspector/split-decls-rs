// Generated macro for impl_373 (impl)
macro_rules! Depcrate_autoderefimpl_373 {
() => {
// Module: crate::autoderef
// Provides: {"impl_373"}
// Dependencies: {}
impl TrackAutoderefSteps for Vec < (AutoderefKind , Ty) > { fn len (& self) -> usize { self . len () } fn push (& mut self , kind : AutoderefKind , ty : & Ty) { self . push ((kind , ty . clone ())) ; } }
};
}
