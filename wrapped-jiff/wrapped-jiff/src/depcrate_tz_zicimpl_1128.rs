// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_tz_zicimpl_1128 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1128"}
// Dependencies: {}
impl FromStr for RuleToP { type Err = Error ; fn from_str (to : & str) -> Result < RuleToP , Error > { if to . starts_with ("m") && "maximum" . starts_with (to) { Ok (RuleToP :: Max) } else if to . starts_with ("o") && "only" . starts_with (to) { Ok (RuleToP :: Only) } else { let year = parse_year (to) . map_err (| e | e . context ("failed to parse TO field")) ? ; Ok (RuleToP :: Year { year }) } } }
};
}
