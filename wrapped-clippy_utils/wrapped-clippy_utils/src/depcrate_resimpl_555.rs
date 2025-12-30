// Generated macro for impl_555 (impl)
macro_rules! Depcrate_resimpl_555 {
() => {
// Module: crate::res
// Provides: {"impl_555"}
// Dependencies: {}
impl < 'tcx , T : Copy + MaybeQPath < 'tcx > > MaybeQPath < 'tcx > for & Option < T > { # [inline] fn opt_qpath (self) -> Option < QPathId < 'tcx > > { self . and_then (T :: opt_qpath) } }
};
}
