// Generated macro for Registry (struct)
macro_rules! Depcrate_registryRegistry {
() => {
// Module: crate::registry
// Provides: {"Registry"}
// Dependencies: {}
pub (super) struct Registry { thread_infos : Vec < ThreadInfo > , sleep : Sleep , injected_jobs : Injector < JobRef > , broadcasts : Mutex < Vec < Worker < JobRef > > > , panic_handler : Option < Box < PanicHandler > > , start_handler : Option < Box < StartHandler > > , exit_handler : Option < Box < ExitHandler > > , terminate_count : AtomicUsize , }
};
}
