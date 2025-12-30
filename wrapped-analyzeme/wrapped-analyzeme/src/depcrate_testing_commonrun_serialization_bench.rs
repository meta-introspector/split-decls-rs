// Generated macro for run_serialization_bench (function)
macro_rules! Depcrate_testing_commonrun_serialization_bench {
() => {
// Module: crate::testing_common
// Provides: {"run_serialization_bench"}
// Dependencies: {}
pub fn run_serialization_bench (file_name_stem : & str , num_events : usize , num_threads : usize) { let filestem = mk_filestem (file_name_stem) ; generate_profiling_data (& filestem , num_events , num_threads) ; }
};
}
