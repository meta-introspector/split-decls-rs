// Generated macro for tests (module)
macro_rules! Depcrate_fmt_temporaltests {
() => {
// Module: crate::fmt::temporal
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: string :: ToString ; use crate :: Unit ; use super :: * ; # [test] fn err_temporal_datetime_offset () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date (b"2024-07-15+02") . unwrap_err () , @ r###"parsed value '2024-07-15', but unparsed input "+02" remains (expected no unparsed input)"### ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date (b"2024-07-15-02") . unwrap_err () , @ r###"parsed value '2024-07-15', but unparsed input "-02" remains (expected no unparsed input)"### ,) ; } # [test] fn year_zero () { insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date ("0000-01-01") . unwrap () , @ "0000-01-01" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date ("+000000-01-01") . unwrap () , @ "0000-01-01" ,) ; insta :: assert_snapshot ! (DateTimeParser :: new () . parse_date ("-000000-01-01") . unwrap_err () , @ "failed to parse year in date `-000000-01-01`: year zero must be written without a sign or a positive sign, but not a negative sign" ,) ; } # [test] fn fractional_duration_roundtrip () { let span1 : Span = "Pt843517081,1H" . parse () . unwrap () ; let span2 : Span = span1 . to_string () . parse () . unwrap () ; assert_eq ! (span1 . total (Unit :: Hour) . unwrap () , span2 . total (Unit :: Hour) . unwrap ()) ; } }
};
}
