// Generated macro for impl_719 (impl)
macro_rules! Depcrate_signed_durationimpl_719 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_719"}
// Dependencies: {}
impl core :: str :: FromStr for SignedDuration { type Err = Error ; # [inline] fn from_str (string : & str) -> Result < SignedDuration , Error > { parse_iso_or_friendly (string . as_bytes ()) } }
};
}
