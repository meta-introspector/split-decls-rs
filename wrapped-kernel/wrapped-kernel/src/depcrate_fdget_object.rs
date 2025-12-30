// Generated macro for get_object (function)
macro_rules! Depcrate_fdget_object {
() => {
// Module: crate::fd
// Provides: {"get_object"}
// Dependencies: {}
pub (crate) fn get_object (fd : FileDescriptor ,) -> io :: Result < Arc < async_lock :: RwLock < dyn ObjectInterface > > > { core_scheduler () . get_object (fd) }
};
}
