// Generated macro for ExecutionContext (struct)
macro_rules! Depcrate_utils_execExecutionContext {
() => {
// Module: crate::utils::exec
// Provides: {"ExecutionContext"}
// Dependencies: {}
# [derive (Clone , Default)] pub struct ExecutionContext { dry_run : DryRun , pub verbosity : u8 , pub fail_fast : bool , delayed_failures : Arc < Mutex < Vec < String > > > , command_cache : Arc < CommandCache > , profiler : Arc < CommandProfiler > , }
};
}
