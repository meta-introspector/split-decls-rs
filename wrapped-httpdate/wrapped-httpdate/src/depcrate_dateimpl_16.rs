// Generated macro for impl_16 (impl)
macro_rules! Depcrate_dateimpl_16 {
() => {
// Module: crate::date
// Provides: {"impl_16"}
// Dependencies: {}
impl FromStr for HttpDate { type Err = Error ; fn from_str (s : & str) -> Result < HttpDate , Error > { if ! s . is_ascii () { return Err (Error (())) ; } let x = s . trim () . as_bytes () ; let date = parse_imf_fixdate (x) . or_else (| _ | parse_rfc850_date (x)) . or_else (| _ | parse_asctime (x)) ? ; if ! date . is_valid () { return Err (Error (())) ; } Ok (date) } }
};
}
