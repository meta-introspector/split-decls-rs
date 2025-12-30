// Generated macro for impl_870 (impl)
macro_rules! Depcrate_timestampimpl_870 {
() => {
// Module: crate::timestamp
// Provides: {"impl_870"}
// Dependencies: {}
impl core :: str :: FromStr for Timestamp { type Err = Error ; # [inline] fn from_str (string : & str) -> Result < Timestamp , Error > { DEFAULT_DATETIME_PARSER . parse_timestamp (string) } }
};
}
