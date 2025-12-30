// Generated macro for macro_507 (macro)
macro_rules! Depcrate_uri_testsmacro_507 {
() => {
// Module: crate::uri::tests
// Provides: {"macro_507"}
// Dependencies: {}
test_parse ! { test_uri_parse_path_and_query , "/some/path/here?and=then&hello#and-bye" , [] , scheme = None , authority = None , path = "/some/path/here" , query = Some ("and=then&hello") , host = None , }
};
}
