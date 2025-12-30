// Generated macro for macro_515 (macro)
macro_rules! Depcrate_uri_testsmacro_515 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_515"}
// Dependencies: {}
test_parse ! { test_uri_parse_absolute_with_default_port_https , "https://127.0.0.1:443" , ["https://127.0.0.1:443/"] , scheme = part ! ("https") , authority = part ! ("127.0.0.1:443") , host = Some ("127.0.0.1") , path = "/" , query = None , port = Port :: from_str ("443") . ok () , }
};
}
