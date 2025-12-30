// Generated macro for threads_from_config (function)
macro_rules! Depcrate_core_configthreads_from_config {
() => {
// Module: crate::core::config
// Provides: {"threads_from_config"}
// Dependencies: {}
pub fn threads_from_config (v : u32) -> u32 { match v { 0 => std :: thread :: available_parallelism () . map_or (1 , std :: num :: NonZeroUsize :: get) as u32 , n => n , } }
};
}
