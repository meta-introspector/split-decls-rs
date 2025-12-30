// Generated macro for impl_160 (impl)
macro_rules! Depcrate_workerimpl_160 {
() => {
// Module: crate::worker
// Provides: {"impl_160"}
// Dependencies: {}
impl Default for ServerWorkerConfig { fn default () -> Self { let parallelism = std :: thread :: available_parallelism () . map_or (2 , NonZeroUsize :: get) ; let max_blocking_threads = std :: cmp :: max (512 / parallelism , 1) ; Self { shutdown_timeout : Duration :: from_secs (30) , max_blocking_threads , max_concurrent_connections : 25600 , } } }
};
}
