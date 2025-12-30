// Generated macro for impl_501 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_501 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_501"}
// Dependencies: {}
impl From < Offset > for TimeZoneAnnotation < 'static > { fn from (offset : Offset) -> TimeZoneAnnotation < 'static > { let kind = TimeZoneAnnotationKind :: from (offset) ; TimeZoneAnnotation { kind , critical : false } } }
};
}
