// Generated macro for tests (module)
macro_rules! Depcrate_util_http_datetests {
() => {
// Module: crate::util::http_date
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: HttpDate ; use std :: time :: { Duration , UNIX_EPOCH } ; fn nov_07 () -> HttpDate { HttpDate ((UNIX_EPOCH + Duration :: new (784198117 , 0)) . into ()) } # [test] fn test_display_is_imf_fixdate () { assert_eq ! ("Mon, 07 Nov 1994 08:48:37 GMT" , & nov_07 () . to_string ()) ; } # [test] fn test_imf_fixdate () { assert_eq ! ("Mon, 07 Nov 1994 08:48:37 GMT" . parse ::< HttpDate > () . unwrap () , nov_07 ()) ; } # [test] fn test_rfc_850 () { assert_eq ! ("Monday, 07-Nov-94 08:48:37 GMT" . parse ::< HttpDate > () . unwrap () , nov_07 ()) ; } # [test] fn test_asctime () { assert_eq ! ("Mon Nov  7 08:48:37 1994" . parse ::< HttpDate > () . unwrap () , nov_07 ()) ; } # [test] fn test_no_date () { assert ! ("this-is-no-date" . parse ::< HttpDate > () . is_err ()) ; } }
};
}
