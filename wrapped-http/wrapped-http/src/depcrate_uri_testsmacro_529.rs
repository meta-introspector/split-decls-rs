// Generated macro for macro_529 (macro)
macro_rules! Depcrate_uri_testsmacro_529 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_529"}
// Dependencies: {}
test_parse ! { test_ipv6_shorthand , "http://[::1]/" , [] , scheme = part ! ("http") , authority = part ! ("[::1]") , host = Some ("[::1]") , path = "/" , query = None , port = None , }
};
}
