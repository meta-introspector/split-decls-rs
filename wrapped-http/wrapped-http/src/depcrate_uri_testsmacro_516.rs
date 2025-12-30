// Generated macro for macro_516 (macro)
macro_rules! Depcrate_uri_testsmacro_516 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_516"}
// Dependencies: {}
test_parse ! { test_uri_parse_fragment_questionmark , "http://127.0.0.1/#?" , [] , scheme = part ! ("http") , authority = part ! ("127.0.0.1") , host = Some ("127.0.0.1") , path = "/" , query = None , port = None , }
};
}
