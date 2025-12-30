// Generated macro for impl_487 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_487 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_487"}
// Dependencies: {}
impl From < Timestamp > for Pieces < 'static > { # [inline] fn from (ts : Timestamp) -> Pieces < 'static > { let dt = Offset :: UTC . to_datetime (ts) ; Pieces :: from (dt) . with_offset (PiecesOffset :: Zulu) } }
};
}
