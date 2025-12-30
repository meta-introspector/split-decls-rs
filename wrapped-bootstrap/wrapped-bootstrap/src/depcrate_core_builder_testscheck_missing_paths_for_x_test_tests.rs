// Generated macro for check_missing_paths_for_x_test_tests (function)
macro_rules! Depcrate_core_builder_testscheck_missing_paths_for_x_test_tests {
() => {
// Module: crate::core::builder::tests
// Provides: {"check_missing_paths_for_x_test_tests"}
// Dependencies: {}
# [test] fn check_missing_paths_for_x_test_tests () { let build = Build :: new (configure ("test" , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1])) ; let (_ , tests_remap_paths) = PATH_REMAP . iter () . find (| (target_path , _) | * target_path == "tests") . unwrap () ; let tests_dir = fs :: read_dir (build . src . join ("tests")) . unwrap () ; for dir in tests_dir { let path = dir . unwrap () . path () ; if path . ends_with ("tests/auxiliary") || ! path . is_dir () { continue ; } assert ! (tests_remap_paths . iter () . any (| item | path . ends_with (* item)) , "{} is missing in PATH_REMAP tests list." , path . display ()) ; } }
};
}
