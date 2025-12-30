// Generated macro for prepare_test_specific_dir (function)
macro_rules! Depcrate_core_config_testsprepare_test_specific_dir {
() => {
// Module: crate::core::config::tests
// Provides: {"prepare_test_specific_dir"}
// Dependencies: {}
# [doc = " Helps with debugging by using consistent test-specific directories instead of"] # [doc = " random temporary directories."] fn prepare_test_specific_dir () -> PathBuf { let current = std :: thread :: current () ; let test_path = current . name () . unwrap () . replace ("::" , "_") ; let testdir = parse ("") . tempdir () . join (test_path) ; let _ = fs :: remove_dir_all (& testdir) ; let _ = fs :: create_dir_all (& testdir) ; testdir }
};
}
