// Generated macro for macro_509 (macro)
macro_rules! Depcrate_uri_testsmacro_509 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_509"}
// Dependencies: {}
test_parse ! { test_uri_parse_absolute_form_without_path , "https://127.0.0.1:61761" , ["https://127.0.0.1:61761/"] , scheme = part ! ("https") , authority = part ! ("127.0.0.1:61761") , path = "/" , query = None , host = Some ("127.0.0.1") , port = Port :: from_str ("61761") . ok () , }
};
}
