// Generated macro for impl_33 (impl)
macro_rules! Depcrate_civil_dateimpl_33 {
() => {
// Module: crate::civil::date
// Provides: {"impl_33"}
// Dependencies: {}
impl core :: str :: FromStr for Date { type Err = Error ; fn from_str (string : & str) -> Result < Date , Error > { DEFAULT_DATETIME_PARSER . parse_date (string) } }
};
}
