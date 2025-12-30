// Generated macro for macro_514 (macro)
macro_rules! Depcrate_uri_testsmacro_514 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_514"}
// Dependencies: {}
test_parse ! { test_uri_parse_absolute_with_default_port_http , "http://127.0.0.1:80" , ["http://127.0.0.1:80/"] , scheme = part ! ("http") , authority = part ! ("127.0.0.1:80") , host = Some ("127.0.0.1") , path = "/" , query = None , port = Port :: from_str ("80") . ok () , }
};
}
