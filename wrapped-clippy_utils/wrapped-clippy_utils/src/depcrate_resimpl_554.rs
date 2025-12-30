// Generated macro for impl_554 (impl)
macro_rules! Depcrate_resimpl_554 {
() => {
// Module: crate::res
// Provides: {"impl_554"}
// Dependencies: {}
impl < 'tcx , T : MaybeQPath < 'tcx > > MaybeQPath < 'tcx > for Option < T > { # [inline] fn opt_qpath (self) -> Option < QPathId < 'tcx > > { self . and_then (T :: opt_qpath) } }
};
}
