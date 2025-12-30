// Generated macro for verify_file_integrity (function)
macro_rules! Depcrate_core_config_testsverify_file_integrity {
() => {
// Module: crate::core::config::tests
// Provides: {"verify_file_integrity"}
// Dependencies: {}
# [test] fn verify_file_integrity () { let config = parse ("") ; let tempfile = config . tempdir () . join (".tmp-test-file") ; File :: create (& tempfile) . unwrap () . write_all (b"dummy value") . unwrap () ; assert ! (tempfile . exists ()) ; assert ! (config . verify (& tempfile , "7e255dd9542648a8779268a0f268b891a198e9828e860ed23f826440e786eae5")) ; remove_file (tempfile) . unwrap () ; }
};
}
