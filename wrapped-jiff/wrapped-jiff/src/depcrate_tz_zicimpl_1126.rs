// Generated macro for impl_1126 (impl)
macro_rules! Depcrate_tz_zicimpl_1126 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1126"}
// Dependencies: {}
impl FromStr for RuleFromP { type Err = Error ; fn from_str (from : & str) -> Result < RuleFromP , Error > { let year = parse_year (from) . map_err (| e | e . context ("failed to parse FROM field")) ? ; Ok (RuleFromP { year }) } }
};
}
