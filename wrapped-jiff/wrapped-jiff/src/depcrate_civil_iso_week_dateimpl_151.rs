// Generated macro for impl_151 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_151 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_151"}
// Dependencies: {}
impl core :: str :: FromStr for ISOWeekDate { type Err = Error ; fn from_str (string : & str) -> Result < ISOWeekDate , Error > { DEFAULT_DATETIME_PARSER . parse_iso_week_date (string) } }
};
}
