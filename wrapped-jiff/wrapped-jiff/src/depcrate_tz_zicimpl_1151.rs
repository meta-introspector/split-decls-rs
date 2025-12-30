// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_tz_zicimpl_1151 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1151"}
// Dependencies: {}
impl FromStr for ZoneRulesP { type Err = Error ; fn from_str (rules : & str) -> Result < ZoneRulesP , Error > { if rules . starts_with (| ch : char | ch == '-' || ch . is_ascii_digit ()) { if rules == "-" { Ok (ZoneRulesP :: None) } else { Ok (ZoneRulesP :: Save (rules . parse () ?)) } } else { Ok (ZoneRulesP :: Named (rules . parse () ?)) } } }
};
}
