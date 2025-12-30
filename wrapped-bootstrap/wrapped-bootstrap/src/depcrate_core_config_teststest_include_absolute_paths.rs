// Generated macro for test_include_absolute_paths (function)
macro_rules! Depcrate_core_config_teststest_include_absolute_paths {
() => {
// Module: crate::core::config::tests
// Provides: {"test_include_absolute_paths"}
// Dependencies: {}
# [test] fn test_include_absolute_paths () { let testdir = prepare_test_specific_dir () ; let extension = testdir . join ("extension.toml") ; File :: create (& extension) . unwrap () . write_all (& []) . unwrap () ; let root_config = testdir . join ("config.toml") ; let extension_absolute_path = extension . canonicalize () . unwrap () . to_str () . unwrap () . replace ('\\' , r"\\") ; let root_config_content = format ! (r#"include = ["{}"]"# , extension_absolute_path) ; File :: create (& root_config) . unwrap () . write_all (root_config_content . as_bytes ()) . unwrap () ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , format ! ("--config={}" , root_config . to_str () . unwrap ())]) , get_toml ,) ; }
};
}
