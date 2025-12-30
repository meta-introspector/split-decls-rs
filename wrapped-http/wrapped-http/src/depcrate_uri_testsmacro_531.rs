// Generated macro for macro_531 (macro)
macro_rules! Depcrate_uri_testsmacro_531 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_531"}
// Dependencies: {}
test_parse ! { test_ipv6_shorthand3 , "http://[2001:db8::2:1]/" , [] , scheme = part ! ("http") , authority = part ! ("[2001:db8::2:1]") , host = Some ("[2001:db8::2:1]") , path = "/" , query = None , port = None , }
};
}
