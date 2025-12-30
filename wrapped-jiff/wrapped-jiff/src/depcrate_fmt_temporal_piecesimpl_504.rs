// Generated macro for impl_504 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_504 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_504"}
// Dependencies: {}
impl < 'n > From < & 'n str > for TimeZoneAnnotationKind < 'n > { fn from (string : & 'n str) -> TimeZoneAnnotationKind < 'n > { let name = TimeZoneAnnotationName :: from (string) ; TimeZoneAnnotationKind :: Named (name) } }
};
}
