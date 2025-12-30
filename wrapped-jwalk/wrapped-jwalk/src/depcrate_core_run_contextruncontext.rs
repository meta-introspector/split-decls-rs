// Generated macro for RunContext (struct)
macro_rules! Depcrate_core_run_contextRunContext {
() => {
// Module: crate::core::run_context
// Provides: {"RunContext"}
// Dependencies: {}
pub (crate) struct RunContext < C : ClientState > { pub (crate) stop : Arc < AtomicBool > , pub (crate) read_dir_spec_queue : OrderedQueue < ReadDirSpec < C > > , pub (crate) read_dir_result_queue : OrderedQueue < Result < ReadDir < C > > > , pub (crate) core_read_dir_callback : Arc < ReadDirCallback < C > > , }
};
}
