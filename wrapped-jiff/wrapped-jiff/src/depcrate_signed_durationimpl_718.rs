// Generated macro for impl_718 (impl)
macro_rules! Depcrate_signed_durationimpl_718 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_718"}
// Dependencies: {}
impl From < Offset > for SignedDuration { fn from (offset : Offset) -> SignedDuration { SignedDuration :: from_secs (i64 :: from (offset . seconds ())) } }
};
}
