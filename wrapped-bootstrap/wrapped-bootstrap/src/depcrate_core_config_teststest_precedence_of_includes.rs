// Generated macro for test_precedence_of_includes (function)
macro_rules! Depcrate_core_config_teststest_precedence_of_includes {
() => {
// Module: crate::core::config::tests
// Provides: {"test_precedence_of_includes"}
// Dependencies: {}
# [test] fn test_precedence_of_includes () { let testdir = prepare_test_specific_dir () ; let root_config = testdir . join ("config.toml") ; let root_config_content = br#"
        include = ["./extension.toml"]

        [llvm]
        link-jobs = 2
    "# ; File :: create (& root_config) . unwrap () . write_all (root_config_content) . unwrap () ; let extension = testdir . join ("extension.toml") ; let extension_content = br#"
        change-id=543
        include = ["./extension2.toml"]
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let extension = testdir . join ("extension2.toml") ; let extension_content = br#"
        change-id=742

        [llvm]
        link-jobs = 10

        [build]
        description = "Some creative description"
    "# ; File :: create (extension) . unwrap () . write_all (extension_content) . unwrap () ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , format ! ("--config={}" , root_config . to_str () . unwrap ())]) , get_toml ,) ; assert_eq ! (config . change_id . unwrap () , ChangeId :: Id (543)) ; assert_eq ! (config . llvm_link_jobs . unwrap () , 2) ; assert_eq ! (config . description . unwrap () , "Some creative description") ; }
};
}
