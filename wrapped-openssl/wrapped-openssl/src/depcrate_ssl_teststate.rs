// Generated macro for state (function)
macro_rules! Depcrate_ssl_teststate {
() => {
// Module: crate::ssl::test
// Provides: {"state"}
// Dependencies: {}
# [test] fn state () { const EXPECTED_STATE_STRING_LONG : & str = "SSL negotiation finished successfully" ; let server = Server :: builder () . build () ; let s = server . client () . connect () ; # [cfg (not (any (boringssl , awslc)))] assert_eq ! (s . ssl () . state_string () . trim () , "SSLOK") ; # [cfg (boringssl)] assert_eq ! (s . ssl () . state_string () , "!!!!!!") ; # [cfg (awslc)] assert_eq ! (s . ssl () . state_string () , EXPECTED_STATE_STRING_LONG) ; assert_eq ! (s . ssl () . state_string_long () , EXPECTED_STATE_STRING_LONG) ; }
};
}
