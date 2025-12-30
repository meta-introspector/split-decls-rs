// Generated macro for impl_1133 (impl)
macro_rules! Depcrate_tz_zicimpl_1133 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1133"}
// Dependencies: {}
impl FromStr for RuleOnP { type Err = Error ; fn from_str (field : & str) -> Result < RuleOnP , Error > { if field . starts_with ("last") { let weekday = parse_weekday (& field [4 ..]) ? ; Ok (RuleOnP :: Last { weekday }) } else if let Some (i) = field . find ("<=") { let weekday = parse_weekday (& field [.. i]) ? ; let day = parse_day (& field [i + 2 ..]) ? ; Ok (RuleOnP :: OnOrBefore { weekday , day }) } else if let Some (i) = field . find (">=") { let weekday = parse_weekday (& field [.. i]) ? ; let day = parse_day (& field [i + 2 ..]) ? ; Ok (RuleOnP :: OnOrAfter { weekday , day }) } else if field . chars () . all (| ch | ch . is_ascii_digit ()) { let day = parse_day (field) ? ; Ok (RuleOnP :: Day { day }) } else { Err (err ! ("unrecognized format for day-of-month: {field:?}")) } } }
};
}
