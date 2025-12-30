// Generated macro for impl_907 (impl)
macro_rules! Depcrate_fd_eventfdimpl_907 {
() => {
// Module: crate::fd::eventfd
// Provides: {"impl_907"}
// Dependencies: {}
impl EventFd { pub fn new (initval : u64 , flags : EventFlags) -> Self { debug ! ("Create EventFd {initval}, {flags:?}") ; Self { state : Mutex :: new (EventState :: new (initval)) , flags , } } }
};
}
