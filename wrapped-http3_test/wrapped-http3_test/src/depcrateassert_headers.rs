// Generated macro for assert_headers (macro)
macro_rules! Depcrateassert_headers {
() => {
// Module: crate
// Provides: {"assert_headers"}
// Dependencies: {}
# [doc = " Asserts that the Http3Req received response headers match the expected"] # [doc = " response headers."] # [doc = ""] # [doc = " Header values are compared with [`assert_eq!`] and this macro will panic"] # [doc = " similarly."] # [doc = ""] # [doc = " If an expected header is not present this macro will panic and print the"] # [doc = " missing header name.AsMut"] # [doc = ""] # [doc = " [`assert_eq!`]: std/macro.assert.html"] # [macro_export] macro_rules ! assert_headers { ($ req : expr) => ({ if let Some (expect_hdrs) = &$ req . expect_resp_hdrs { for hdr in expect_hdrs { match $ req . resp_hdrs . iter () . find (|& x | x . name () == hdr . name ()) { Some (h) => assert_eq ! (hdr . value () , h . value ()) , None => panic ! ("assertion failed: expected response header field {} not present!" , std :: str :: from_utf8 (hdr . name ()) . unwrap ()) , } } } }) ; ($ req : expr ,) => ({ $ crate :: assert_headers ! ($ req) }) ; ($ req : expr , $ ($ arg : tt) +) => ({ if let Some (expect_hdrs) = &$ req . expect_resp_hdrs { for hdr in expect_hdrs { match $ req . resp_hdrs . iter () . find (|& x | x . name () == hdr . name ()) { Some (h) => { assert_eq ! (hdr . value () , h . value () , $ ($ arg) +) ; } , None => { panic ! ("assertion failed: expected response header field {} not present! {}" , hdr . name () , $ ($ arg) +) ; } } } } }) ; }
};
}
