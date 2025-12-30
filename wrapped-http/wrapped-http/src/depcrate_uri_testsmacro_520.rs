// Generated macro for macro_520 (macro)
macro_rules! Depcrate_uri_testsmacro_520 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_520"}
// Dependencies: {}
test_parse ! { test_uri_parse_absolute_form_with_empty_path_and_fragment_with_questionmark , "http://127.0.0.1#foo?bar" , [] , scheme = part ! ("http") , authority = part ! ("127.0.0.1") , path = "/" , query = None , port = None , }
};
}
