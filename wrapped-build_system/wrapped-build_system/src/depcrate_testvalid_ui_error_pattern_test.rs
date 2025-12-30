// Generated macro for valid_ui_error_pattern_test (function)
macro_rules! Depcrate_testvalid_ui_error_pattern_test {
() => {
// Module: crate::test
// Provides: {"valid_ui_error_pattern_test"}
// Dependencies: {}
fn valid_ui_error_pattern_test (file : & str) -> bool { ["issues/auxiliary/issue-3136-a.rs" , "type-alias-impl-trait/auxiliary/cross_crate_ice.rs" , "type-alias-impl-trait/auxiliary/cross_crate_ice2.rs" , "macros/rfc-2011-nicer-assert-messages/auxiliary/common.rs" , "imports/ambiguous-1.rs" , "imports/ambiguous-4-extern.rs" , "entry-point/auxiliary/bad_main_functions.rs" ,] . iter () . any (| to_ignore | file . ends_with (to_ignore)) }
};
}
