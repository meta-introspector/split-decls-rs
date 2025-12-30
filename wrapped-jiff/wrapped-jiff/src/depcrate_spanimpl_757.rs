// Generated macro for impl_757 (impl)
macro_rules! Depcrate_spanimpl_757 {
() => {
// Module: crate::span
// Provides: {"impl_757"}
// Dependencies: {}
impl core :: str :: FromStr for Span { type Err = Error ; # [inline] fn from_str (string : & str) -> Result < Span , Error > { parse_iso_or_friendly (string . as_bytes ()) } }
};
}
