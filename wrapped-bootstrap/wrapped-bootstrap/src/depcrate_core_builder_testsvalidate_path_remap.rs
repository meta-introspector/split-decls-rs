// Generated macro for validate_path_remap (function)
macro_rules! Depcrate_core_builder_testsvalidate_path_remap {
() => {
// Module: crate::core::builder::tests
// Provides: {"validate_path_remap"}
// Dependencies: {}
# [test] fn validate_path_remap () { let build = Build :: new (configure ("test" , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1])) ; PATH_REMAP . iter () . flat_map (| (_ , paths) | paths . iter ()) . map (| path | build . src . join (path)) . for_each (| path | { assert ! (path . exists () , "{} should exist." , path . display ()) ; }) ; }
};
}
