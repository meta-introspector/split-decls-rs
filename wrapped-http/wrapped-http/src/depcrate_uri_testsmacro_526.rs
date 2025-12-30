// Generated macro for macro_526 (macro)
macro_rules! Depcrate_uri_testsmacro_526 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_526"}
// Dependencies: {}
test_parse ! { test_userinfo_with_port , "user@localhost:3000" , [] , scheme = None , authority = part ! ("user@localhost:3000") , path = "" , query = None , host = Some ("localhost") , port = Port :: from_str ("3000") . ok () , }
};
}
