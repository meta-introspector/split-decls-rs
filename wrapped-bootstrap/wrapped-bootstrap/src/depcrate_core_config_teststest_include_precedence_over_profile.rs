// Generated macro for test_include_precedence_over_profile (function)
macro_rules! Depcrate_core_config_teststest_include_precedence_over_profile {
() => {
// Module: crate::core::config::tests
// Provides: {"test_include_precedence_over_profile"}
// Dependencies: {}
# [test] fn test_include_precedence_over_profile () { let testdir = prepare_test_specific_dir () ; let root_config = testdir . join ("config.toml") ; let root_config_content = br#"
        profile = "dist"
        include = ["./extension.toml"]
    "# ; File :: create (& root_config) . unwrap () . write_all (root_config_content) . unwrap () ; let extension = testdir . join ("extension.toml") ; let extension_content = br#"
        [rust]
        channel = "dev"
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , format ! ("--config={}" , root_config . to_str () . unwrap ())]) , get_toml ,) ; assert_eq ! (config . channel , "dev") ; }
};
}
