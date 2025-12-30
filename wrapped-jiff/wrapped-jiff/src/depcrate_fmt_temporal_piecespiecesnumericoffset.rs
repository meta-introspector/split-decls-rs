// Generated macro for PiecesNumericOffset (struct)
macro_rules! Depcrate_fmt_temporal_piecesPiecesNumericOffset {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"PiecesNumericOffset"}
// Dependencies: {}
# [doc = " A specific numeric offset, including the sign of the offset, for use with"] # [doc = " [`Pieces`]."] # [doc = ""] # [doc = " # Signedness"] # [doc = ""] # [doc = " The sign attached to this type is usually redundant, since the underlying"] # [doc = " [`Offset`] is itself signed. But it can be used to distinguish between"] # [doc = " `+00:00` (`+00` is the preferred offset) and `-00:00` (`+00` is what should"] # [doc = " be used, but only because the offset to local time is not known). Generally"] # [doc = " speaking, one should regard `-00:00` as equivalent to `Z`, per [RFC 9557]."] # [doc = ""] # [doc = " [RFC 9557]: https://www.rfc-editor.org/rfc/rfc9557"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct PiecesNumericOffset { offset : Offset , is_negative : bool , }
};
}
