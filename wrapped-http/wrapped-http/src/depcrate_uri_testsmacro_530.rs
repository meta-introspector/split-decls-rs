// Generated macro for macro_530 (macro)
macro_rules! Depcrate_uri_testsmacro_530 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_530"}
// Dependencies: {}
test_parse ! { test_ipv6_shorthand2 , "http://[::]/" , [] , scheme = part ! ("http") , authority = part ! ("[::]") , host = Some ("[::]") , path = "/" , query = None , port = None , }
};
}
