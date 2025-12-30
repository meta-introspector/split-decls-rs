// Generated macro for impl_1141 (impl)
macro_rules! Depcrate_tz_zicimpl_1141 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1141"}
// Dependencies: {}
impl FromStr for RuleSaveP { type Err = Error ; fn from_str (at : & str) -> Result < RuleSaveP , Error > { if at . is_empty () { return Err (err ! ("empty field is not a valid SAVE value")) ; } let (span_string , suffix_string) = at . split_at (at . len () - 1) ; if suffix_string . chars () . all (| ch | ch . is_ascii_alphabetic ()) { let span = parse_span (span_string) ? . fieldwise () ; let suffix = suffix_string . parse () ? ; Ok (RuleSaveP { span , suffix : Some (suffix) }) } else { let span = parse_span (at) ? . fieldwise () ; Ok (RuleSaveP { span , suffix : None }) } } }
};
}
