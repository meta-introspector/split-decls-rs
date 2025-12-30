// Generated macro for macro_527 (macro)
macro_rules! Depcrate_uri_testsmacro_527 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_527"}
// Dependencies: {}
test_parse ! { test_userinfo_pass_with_port , "user:pass@localhost:3000" , [] , scheme = None , authority = part ! ("user:pass@localhost:3000") , path = "" , query = None , host = Some ("localhost") , port = Port :: from_str ("3000") . ok () , }
};
}
