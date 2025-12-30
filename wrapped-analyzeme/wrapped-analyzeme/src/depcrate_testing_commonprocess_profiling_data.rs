// Generated macro for process_profiling_data (function)
macro_rules! Depcrate_testing_commonprocess_profiling_data {
() => {
// Module: crate::testing_common
// Provides: {"process_profiling_data"}
// Dependencies: {}
fn process_profiling_data (filestem : & Path , expected_events : & [Event < 'static >]) { let profiling_data = ProfilingData :: new (filestem) . unwrap () ; check_profiling_data (& mut profiling_data . iter_full () , & mut expected_events . iter () . cloned () , expected_events . len () ,) ; check_profiling_data (& mut profiling_data . iter_full () . rev () , & mut expected_events . iter () . rev () . cloned () , expected_events . len () ,) ; }
};
}
