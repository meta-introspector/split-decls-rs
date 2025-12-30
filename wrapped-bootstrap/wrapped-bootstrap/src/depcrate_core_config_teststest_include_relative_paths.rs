// Generated macro for test_include_relative_paths (function)
macro_rules! Depcrate_core_config_teststest_include_relative_paths {
() => {
// Module: crate::core::config::tests
// Provides: {"test_include_relative_paths"}
// Dependencies: {}
# [test] fn test_include_relative_paths () { let testdir = prepare_test_specific_dir () ; let _ = fs :: create_dir_all (& testdir . join ("subdir/another_subdir")) ; let root_config = testdir . join ("config.toml") ; let root_config_content = br#"
        include = ["./subdir/extension.toml"]
    "# ; File :: create (& root_config) . unwrap () . write_all (root_config_content) . unwrap () ; let extension = testdir . join ("subdir/extension.toml") ; let extension_content = br#"
        include = ["../extension2.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let extension = testdir . join ("extension2.toml") ; let extension_content = br#"
        include = ["./subdir/another_subdir/extension3.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let extension = testdir . join ("subdir/another_subdir/extension3.toml") ; let extension_content = br#"
        include = ["../../extension4.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let extension = testdir . join ("extension4.toml") ; File :: create (extension) . unwrap () . write_all (& []) . unwrap () ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , format ! ("--config={}" , root_config . to_str () . unwrap ())]) , get_toml ,) ; }
};
}
