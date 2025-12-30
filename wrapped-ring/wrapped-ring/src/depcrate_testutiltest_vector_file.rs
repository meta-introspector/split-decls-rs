// Generated macro for test_vector_file (macro)
macro_rules! Depcrate_testutiltest_vector_file {
() => {
// Module: crate::testutil
// Provides: {"test_vector_file"}
// Dependencies: {}
# [doc = " References a test input file."] # [cfg (test)] macro_rules ! test_vector_file { ($ file_name : expr) => { $ crate :: testutil :: File { file_name : $ file_name , contents : include_str ! ($ file_name) , } } ; }
};
}
