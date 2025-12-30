// Generated macro for impl_1136 (impl)
macro_rules! Depcrate_tz_zicimpl_1136 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1136"}
// Dependencies: {}
impl FromStr for RuleAtP { type Err = Error ; fn from_str (at : & str) -> Result < RuleAtP , Error > { if at . is_empty () { return Err (err ! ("empty field is not a valid AT value")) ; } let (span_string , suffix_string) = at . split_at (at . len () - 1) ; if suffix_string . chars () . all (| ch | ch . is_ascii_alphabetic ()) { let span = parse_span (span_string) ? . fieldwise () ; let suffix = suffix_string . parse () ? ; Ok (RuleAtP { span , suffix : Some (suffix) }) } else { let span = parse_span (at) ? . fieldwise () ; Ok (RuleAtP { span , suffix : None }) } } }
};
}
