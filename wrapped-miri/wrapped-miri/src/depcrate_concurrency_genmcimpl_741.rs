// Generated macro for impl_741 (impl)
macro_rules! Depcrate_concurrency_genmcimpl_741 {
() => {
// Module: crate::concurrency::genmc
// Provides: {"impl_741"}
// Dependencies: {}
impl PerExecutionState { fn reset (& self) { self . allow_data_races . replace (false) ; self . thread_id_manager . borrow_mut () . reset () ; self . exit_status . set (None) ; self . genmc_shared_allocs_map . borrow_mut () . clear () ; } }
};
}
