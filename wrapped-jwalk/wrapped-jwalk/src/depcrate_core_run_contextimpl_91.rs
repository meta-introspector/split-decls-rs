// Generated macro for impl_91 (impl)
macro_rules! Depcrate_core_run_contextimpl_91 {
() => {
// Module: crate::core::run_context
// Provides: {"impl_91"}
// Dependencies: {}
impl < C : ClientState > RunContext < C > { pub (crate) fn stop (& self) { self . stop . store (true , AtomicOrdering :: SeqCst) ; } pub (crate) fn schedule_read_dir_spec (& self , ordered_read_dir : Ordered < ReadDirSpec < C > >) -> bool { self . read_dir_spec_queue . push (ordered_read_dir) . is_ok () } pub (crate) fn send_read_dir_result (& self , read_dir_result : Ordered < Result < ReadDir < C > > > ,) -> bool { self . read_dir_result_queue . push (read_dir_result) . is_ok () } pub (crate) fn complete_item (& self) { self . read_dir_spec_queue . complete_item () } }
};
}
