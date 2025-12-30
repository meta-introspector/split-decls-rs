// Generated macro for impl_500 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_500 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_500"}
// Dependencies: {}
impl < 'n > From < & 'n str > for TimeZoneAnnotation < 'n > { fn from (string : & 'n str) -> TimeZoneAnnotation < 'n > { let kind = TimeZoneAnnotationKind :: from (string) ; TimeZoneAnnotation { kind , critical : false } } }
};
}
