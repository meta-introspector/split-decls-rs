// Generated macro for macro_513 (macro)
macro_rules! Depcrate_uri_testsmacro_513 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_513"}
// Dependencies: {}
test_parse ! { test_uri_parse_authority_form , "localhost:3000" , ["localhosT:3000"] , scheme = None , authority = part ! ("localhost:3000") , path = "" , query = None , host = Some ("localhost") , port = Port :: from_str ("3000") . ok () , }
};
}
