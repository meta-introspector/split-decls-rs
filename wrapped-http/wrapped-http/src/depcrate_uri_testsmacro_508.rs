// Generated macro for macro_508 (macro)
macro_rules! Depcrate_uri_testsmacro_508 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_508"}
// Dependencies: {}
test_parse ! { test_uri_parse_absolute_form , "http://127.0.0.1:61761/chunks" , [] , scheme = part ! ("http") , authority = part ! ("127.0.0.1:61761") , path = "/chunks" , query = None , host = Some ("127.0.0.1") , port = Port :: from_str ("61761") . ok () , }
};
}
