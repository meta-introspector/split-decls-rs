// Generated macro for impl_738 (impl)
macro_rules! Depcrate_signed_durationimpl_738 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_738"}
// Dependencies: {}
impl From < (Unit , i64) > for SignedDurationRound { fn from ((unit , increment) : (Unit , i64)) -> SignedDurationRound { SignedDurationRound :: default () . smallest (unit) . increment (increment) } }
};
}
