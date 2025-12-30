// Generated macro for macro_523 (macro)
macro_rules! Depcrate_uri_testsmacro_523 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_523"}
// Dependencies: {}
test_parse ! { test_userinfo1 , "http://a:b@127.0.0.1:1234/" , [] , scheme = part ! ("http") , authority = part ! ("a:b@127.0.0.1:1234") , host = Some ("127.0.0.1") , path = "/" , query = None , port = Port :: from_str ("1234") . ok () , }
};
}
