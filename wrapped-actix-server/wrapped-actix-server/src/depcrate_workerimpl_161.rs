// Generated macro for impl_161 (impl)
macro_rules! Depcrate_workerimpl_161 {
() => {
// Module: crate::worker
// Provides: {"impl_161"}
// Dependencies: {}
impl ServerWorkerConfig { pub (crate) fn max_blocking_threads (& mut self , num : usize) { self . max_blocking_threads = num ; } pub (crate) fn max_concurrent_connections (& mut self , num : usize) { self . max_concurrent_connections = num ; } pub (crate) fn shutdown_timeout (& mut self , dur : Duration) { self . shutdown_timeout = dur ; } }
};
}
