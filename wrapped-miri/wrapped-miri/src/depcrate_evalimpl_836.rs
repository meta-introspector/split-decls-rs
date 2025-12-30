// Generated macro for impl_836 (impl)
macro_rules! Depcrate_evalimpl_836 {
() => {
// Module: crate::eval
// Provides: {"impl_836"}
// Dependencies: {}
impl Default for MiriConfig { fn default () -> MiriConfig { MiriConfig { env : vec ! [] , validation : ValidationMode :: Shallow , borrow_tracker : Some (BorrowTrackerMethod :: StackedBorrows) , check_alignment : AlignmentCheck :: Int , isolated_op : IsolatedOp :: Reject (RejectOpWith :: Abort) , ignore_leaks : false , forwarded_env_vars : vec ! [] , set_env_vars : FxHashMap :: default () , args : vec ! [] , seed : None , tracked_pointer_tags : FxHashSet :: default () , tracked_alloc_ids : FxHashSet :: default () , track_alloc_accesses : false , data_race_detector : true , weak_memory_emulation : true , genmc_config : None , track_outdated_loads : false , cmpxchg_weak_failure_rate : 0.8 , measureme_out : None , backtrace_style : BacktraceStyle :: Short , provenance_mode : ProvenanceMode :: Default , mute_stdout_stderr : false , preemption_rate : 0.01 , report_progress : None , native_lib : vec ! [] , native_lib_enable_tracing : false , gc_interval : 10_000 , num_cpus : 1 , page_size : None , collect_leak_backtraces : true , address_reuse_rate : 0.5 , address_reuse_cross_thread_rate : 0.1 , fixed_scheduling : false , force_intrinsic_fallback : false , float_nondet : true , float_rounding_error : FloatRoundingErrorMode :: Random , short_fd_operations : true , user_relevant_crates : vec ! [] , } } }
};
}
