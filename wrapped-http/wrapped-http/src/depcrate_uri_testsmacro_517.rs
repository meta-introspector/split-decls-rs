// Generated macro for macro_517 (macro)
macro_rules! Depcrate_uri_testsmacro_517 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_517"}
// Dependencies: {}
test_parse ! { test_uri_parse_path_with_terminating_questionmark , "http://127.0.0.1/path?" , [] , scheme = part ! ("http") , authority = part ! ("127.0.0.1") , path = "/path" , query = Some ("") , port = None , }
};
}
