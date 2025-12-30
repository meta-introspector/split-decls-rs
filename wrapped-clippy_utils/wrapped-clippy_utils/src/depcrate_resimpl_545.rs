// Generated macro for impl_545 (impl)
macro_rules! Depcrate_resimpl_545 {
() => {
// Module: crate::res
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'tcx > MaybeTypeckRes < 'tcx > for LateContext < 'tcx > { # [inline] # [cfg_attr (debug_assertions , track_caller)] fn typeck_res (& self) -> Option < & TypeckResults < 'tcx > > { if let Some (typeck) = self . maybe_typeck_results () { Some (typeck) } else { debug_assert ! (false , "attempted type-dependent lookup in a non-body context") ; None } } }
};
}
