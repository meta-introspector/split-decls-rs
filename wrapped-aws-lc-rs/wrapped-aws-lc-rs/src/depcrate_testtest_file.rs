// Generated macro for test_file (macro)
macro_rules! Depcrate_testtest_file {
() => {
// Module: crate::test
// Provides: {"test_file"}
// Dependencies: {}
# [doc = " References a test input file."] # [macro_export] # [allow (clippy :: module_name_repetitions)] macro_rules ! test_file { ($ file_name : expr) => { $ crate :: test :: File { file_name : $ file_name , contents : include_str ! ($ file_name) , } } ; }
};
}
