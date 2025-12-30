// Generated macro for run_end_to_end_serialization_test (function)
macro_rules! Depcrate_testing_commonrun_end_to_end_serialization_test {
() => {
// Module: crate::testing_common
// Provides: {"run_end_to_end_serialization_test"}
// Dependencies: {}
pub fn run_end_to_end_serialization_test (file_name_stem : & str , num_threads : usize) { let filestem = mk_filestem (file_name_stem) ; let expected_events = generate_profiling_data (& filestem , 10_000 , num_threads) ; process_profiling_data (& filestem , & expected_events) ; }
};
}
