// Generated macro for impl_1145 (impl)
macro_rules! Depcrate_tz_zicimpl_1145 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1145"}
// Dependencies: {}
impl FromStr for RuleLettersP { type Err = Error ; fn from_str (letters : & str) -> Result < RuleLettersP , Error > { let part = if letters == "-" { String :: new () } else { letters . to_string () } ; Ok (RuleLettersP { part }) } }
};
}
