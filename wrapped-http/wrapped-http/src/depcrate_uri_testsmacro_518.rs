// Generated macro for macro_518 (macro)
macro_rules! Depcrate_uri_testsmacro_518 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_518"}
// Dependencies: {}
test_parse ! { test_uri_parse_absolute_form_with_empty_path_and_nonempty_query , "http://127.0.0.1?foo=bar" , [] , scheme = part ! ("http") , authority = part ! ("127.0.0.1") , path = "/" , query = Some ("foo=bar") , port = None , }
};
}
