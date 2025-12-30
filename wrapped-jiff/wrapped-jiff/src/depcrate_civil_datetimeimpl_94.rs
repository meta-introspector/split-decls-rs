// Generated macro for impl_94 (impl)
macro_rules! Depcrate_civil_datetimeimpl_94 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_94"}
// Dependencies: {}
impl core :: str :: FromStr for DateTime { type Err = Error ; # [inline] fn from_str (string : & str) -> Result < DateTime , Error > { DEFAULT_DATETIME_PARSER . parse_datetime (string) } }
};
}
