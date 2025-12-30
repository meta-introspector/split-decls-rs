// Generated macro for impl_905 (impl)
macro_rules! Depcrate_fd_eventfdimpl_905 {
() => {
// Module: crate::fd::eventfd
// Provides: {"impl_905"}
// Dependencies: {}
impl EventState { pub fn new (counter : u64) -> Self { Self { counter , read_queue : VecDeque :: new () , write_queue : VecDeque :: new () , } } }
};
}
