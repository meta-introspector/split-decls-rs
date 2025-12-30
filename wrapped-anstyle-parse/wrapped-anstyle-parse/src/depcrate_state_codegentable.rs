// Generated macro for table (function)
macro_rules! Depcrate_state_codegentable {
() => {
// Module: crate::state::codegen
// Provides: {"table"}
// Dependencies: {}
# [test] fn table () { let mut content = vec ! [] ; generate_table (& mut content) . unwrap () ; let content = String :: from_utf8 (content) . unwrap () ; let content = codegenrs :: rustfmt (& content , None) . unwrap () ; snapbox :: assert_data_eq ! (content , file ! ["table.rs"] . raw ()) ; }
};
}
