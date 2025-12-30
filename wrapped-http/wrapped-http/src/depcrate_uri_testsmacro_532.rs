// Generated macro for macro_532 (macro)
macro_rules! Depcrate_uri_testsmacro_532 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_532"}
// Dependencies: {}
test_parse ! { test_ipv6_with_port , "http://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:8008/" , [] , scheme = part ! ("http") , authority = part ! ("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:8008") , host = Some ("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]") , path = "/" , query = None , port = Port :: from_str ("8008") . ok () , }
};
}
