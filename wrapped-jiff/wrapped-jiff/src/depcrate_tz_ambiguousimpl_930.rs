// Generated macro for impl_930 (impl)
macro_rules! Depcrate_tz_ambiguousimpl_930 {
() => {
// Module: crate::tz::ambiguous
// Provides: {"impl_930"}
// Dependencies: {}
impl AmbiguousOffset { # [inline] pub (crate) const fn from_iambiguous_offset_const (iaoff : IAmbiguousOffset ,) -> AmbiguousOffset { match iaoff { IAmbiguousOffset :: Unambiguous { offset } => { let offset = Offset :: from_ioffset_const (offset) ; AmbiguousOffset :: Unambiguous { offset } } IAmbiguousOffset :: Gap { before , after } => { let before = Offset :: from_ioffset_const (before) ; let after = Offset :: from_ioffset_const (after) ; AmbiguousOffset :: Gap { before , after } } IAmbiguousOffset :: Fold { before , after } => { let before = Offset :: from_ioffset_const (before) ; let after = Offset :: from_ioffset_const (after) ; AmbiguousOffset :: Fold { before , after } } } } }
};
}
