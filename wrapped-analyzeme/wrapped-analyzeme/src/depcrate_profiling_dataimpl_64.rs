// Generated macro for impl_64 (impl)
macro_rules! Depcrate_profiling_dataimpl_64 {
() => {
// Module: crate::profiling_data
// Provides: {"impl_64"}
// Dependencies: {}
impl ProfilerFiles { fn new < P : AsRef < Path > > (path_stem : P) -> ProfilerFiles { ProfilerFiles { events_file : path_stem . as_ref () . with_extension ("events") , string_data_file : path_stem . as_ref () . with_extension ("string_data") , string_index_file : path_stem . as_ref () . with_extension ("string_index") , } } }
};
}
