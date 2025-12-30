// Generated macro for PerExecutionState (struct)
macro_rules! Depcrate_concurrency_genmcPerExecutionState {
() => {
// Module: crate::concurrency::genmc
// Provides: {"PerExecutionState"}
// Dependencies: {}
# [doc = " State that is reset at the start of every execution."] # [derive (Debug , Default)] struct PerExecutionState { # [doc = " Thread id management, such as mapping between Miri `ThreadId` and GenMC's thread ids, or selecting GenMC thread ids."] thread_id_manager : RefCell < ThreadIdMap > , # [doc = " A flag to indicate that we should not forward non-atomic accesses to genmc, e.g. because we"] # [doc = " are executing an atomic operation."] allow_data_races : Cell < bool > , # [doc = " The exit status of the program. We keep running other threads even after `exit` to ensure"] # [doc = " we cover all possible executions."] # [doc = " `None` if no thread has called `exit` and the main thread isn't finished yet."] exit_status : Cell < Option < ExitStatus > > , # [doc = " Allocations in this map have been sent to GenMC, and should thus be kept around, since future loads from GenMC may return this allocation again."] genmc_shared_allocs_map : RefCell < FxHashMap < u64 , AllocId > > , }
};
}
