// Generated macro for parse_change_id_with_unknown_field (function)
macro_rules! Depcrate_core_config_testsparse_change_id_with_unknown_field {
() => {
// Module: crate::core::config::tests
// Provides: {"parse_change_id_with_unknown_field"}
// Dependencies: {}
# [test] fn parse_change_id_with_unknown_field () { let config = r#"
        change-id = 3461
        unknown-key = 1
    "# ; let change_id_wrapper : ChangeIdWrapper = toml :: from_str (config) . unwrap () ; assert_eq ! (change_id_wrapper . inner , Some (ChangeId :: Id (3461))) ; }
};
}
