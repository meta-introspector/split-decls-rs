// Generated macro for impl_503 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_503 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_503"}
// Dependencies: {}
impl < 'n > TimeZoneAnnotationKind < 'n > { # [doc = " Converts this time zone annotation kind into an \"owned\" value whose"] # [doc = " lifetime is `'static`."] # [doc = ""] # [doc = " If this was already an \"owned\" value or a time zone annotation offset,"] # [doc = " then this is a no-op."] # [cfg (feature = "alloc")] # [inline] pub fn into_owned (self) -> TimeZoneAnnotationKind < 'static > { match self { TimeZoneAnnotationKind :: Named (named) => { TimeZoneAnnotationKind :: Named (named . into_owned ()) } TimeZoneAnnotationKind :: Offset (offset) => { TimeZoneAnnotationKind :: Offset (offset) } } } }
};
}
