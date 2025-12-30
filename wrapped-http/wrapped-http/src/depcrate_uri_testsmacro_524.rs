// Generated macro for macro_524 (macro)
macro_rules! Depcrate_uri_testsmacro_524 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_524"}
// Dependencies: {}
test_parse ! { test_userinfo2 , "http://a:b@127.0.0.1/" , [] , scheme = part ! ("http") , authority = part ! ("a:b@127.0.0.1") , host = Some ("127.0.0.1") , path = "/" , query = None , port = None , }
};
}
