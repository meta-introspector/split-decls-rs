// Generated macro for tests (module)
macro_rules! Depcrate_common_retry_aftertests {
() => {
// Module: crate::common::retry_after
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: time :: Duration ; use super :: super :: test_decode ; use super :: RetryAfter ; use crate :: util :: HttpDate ; # [test] fn delay_decode () { let r : RetryAfter = test_decode (& ["1234"]) . unwrap () ; assert_eq ! (r , RetryAfter :: delay (Duration :: from_secs (1234)) ,) ; } macro_rules ! test_retry_after_datetime { ($ name : ident , $ s : expr) => { # [test] fn $ name () { let r : RetryAfter = test_decode (& [$ s]) . unwrap () ; let dt = "Sun, 06 Nov 1994 08:49:37 GMT" . parse ::< HttpDate > () . unwrap () ; assert_eq ! (r , RetryAfter (super :: After :: DateTime (dt))) ; } } ; } test_retry_after_datetime ! (date_decode_rfc1123 , "Sun, 06 Nov 1994 08:49:37 GMT") ; test_retry_after_datetime ! (date_decode_rfc850 , "Sunday, 06-Nov-94 08:49:37 GMT") ; test_retry_after_datetime ! (date_decode_asctime , "Sun Nov  6 08:49:37 1994") ; }
};
}
