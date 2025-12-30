// Generated macro for test_cyclic_include_indirect (function)
macro_rules! Depcrate_core_config_teststest_cyclic_include_indirect {
() => {
// Module: crate::core::config::tests
// Provides: {"test_cyclic_include_indirect"}
// Dependencies: {}
# [test] # [should_panic (expected = "Cyclic inclusion detected")] fn test_cyclic_include_indirect () { let testdir = prepare_test_specific_dir () ; let root_config = testdir . join ("config.toml") ; let root_config_content = br#"
        include = ["./extension.toml"]
    "# ; File :: create (& root_config) . unwrap () . write_all (root_config_content) . unwrap () ; let extension = testdir . join ("extension.toml") ; let extension_content = br#"
        include = ["./extension2.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let extension = testdir . join ("extension2.toml") ; let extension_content = br#"
        include = ["./extension3.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let extension = testdir . join ("extension3.toml") ; let extension_content = br#"
        include = ["./extension.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , format ! ("--config={}" , root_config . to_str () . unwrap ())]) , get_toml ,) ; }
};
}
