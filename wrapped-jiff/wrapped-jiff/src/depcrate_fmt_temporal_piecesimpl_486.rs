// Generated macro for impl_486 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_486 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_486"}
// Dependencies: {}
impl From < DateTime > for Pieces < 'static > { # [inline] fn from (dt : DateTime) -> Pieces < 'static > { Pieces :: from (dt . date ()) . with_time (dt . time ()) } }
};
}
