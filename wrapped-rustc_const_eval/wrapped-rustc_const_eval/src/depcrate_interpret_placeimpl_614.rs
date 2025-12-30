// Generated macro for impl_614 (impl)
macro_rules! Depcrate_interpret_placeimpl_614 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_614"}
// Dependencies: {}
impl < Prov : Provenance > MemPlaceMeta < Prov > { # [cfg_attr (debug_assertions , track_caller)] pub fn unwrap_meta (self) -> Scalar < Prov > { match self { Self :: Meta (s) => s , Self :: None => { bug ! ("expected wide pointer extra data (e.g. slice length or trait object vtable)") } } } # [inline (always)] pub fn has_meta (self) -> bool { match self { Self :: Meta (_) => true , Self :: None => false , } } }
};
}
