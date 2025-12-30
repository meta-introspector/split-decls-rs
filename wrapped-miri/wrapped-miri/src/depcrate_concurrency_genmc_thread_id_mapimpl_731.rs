// Generated macro for impl_731 (impl)
macro_rules! Depcrate_concurrency_genmc_thread_id_mapimpl_731 {
() => {
// Module: crate::concurrency::genmc::thread_id_map
// Provides: {"impl_731"}
// Dependencies: {}
impl Default for ThreadIdMap { fn default () -> Self { let miri_to_genmc = [(ThreadId :: MAIN_THREAD , GENMC_MAIN_THREAD_ID)] . into_iter () . collect () ; let genmc_to_miri = vec ! [ThreadId :: MAIN_THREAD] ; Self { miri_to_genmc , genmc_to_miri } } }
};
}
