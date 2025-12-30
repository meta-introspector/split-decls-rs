// Generated macro for impl_175 (impl)
macro_rules! Depcrate_civil_timeimpl_175 {
() => {
// Module: crate::civil::time
// Provides: {"impl_175"}
// Dependencies: {}
impl core :: str :: FromStr for Time { type Err = Error ; # [inline] fn from_str (string : & str) -> Result < Time , Error > { DEFAULT_DATETIME_PARSER . parse_time (string) } }
};
}
