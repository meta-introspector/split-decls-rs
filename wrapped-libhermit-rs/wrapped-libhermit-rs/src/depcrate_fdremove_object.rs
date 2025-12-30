// Generated macro for remove_object (function)
macro_rules! Depcrate_fdremove_object {
() => {
// Module: crate::fd
// Provides: {"remove_object"}
// Dependencies: {}
pub (crate) fn remove_object (fd : FileDescriptor ,) -> io :: Result < Arc < async_lock :: RwLock < dyn ObjectInterface > > > { core_scheduler () . remove_object (fd) }
};
}
