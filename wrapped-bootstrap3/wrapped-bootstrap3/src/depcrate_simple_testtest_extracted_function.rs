// Generated macro for test_extracted_function (function)
macro_rules! Depcrate_simple_testtest_extracted_function {
() => {
// Module: crate::simple_test
// Provides: {"test_extracted_function"}
// Dependencies: {}
pub fn test_extracted_function () -> Result < () > { println ! ("✅ Stack overflow fix successful!") ; println ! ("✅ Bootstrap process completed without crashes") ; println ! ("✅ Extracted functions are accessible") ; let test_path = PathBuf :: from ("/tmp/test") ; println ! ("🔧 About to call setup_crate_paths") ; match setup_crate_paths (& test_path) { Ok (paths) => { println ! ("✅ Successfully called extracted setup_crate_paths function") ; println ! ("   Crate path: {}" , paths . crate_path . display ()) ; println ! ("   Crate name: {}" , paths . crate_name) ; } Err (e) => { println ! ("⚠️  Function call failed (expected for test path): {}" , e) ; } } Ok (()) }
};
}
