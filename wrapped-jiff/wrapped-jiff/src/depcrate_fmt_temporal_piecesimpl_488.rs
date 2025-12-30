// Generated macro for impl_488 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_488 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_488"}
// Dependencies: {}
impl From < (Timestamp , Offset) > for Pieces < 'static > { # [inline] fn from ((ts , offset) : (Timestamp , Offset)) -> Pieces < 'static > { Pieces :: from (offset . to_datetime (ts)) . with_offset (offset) } }
};
}
