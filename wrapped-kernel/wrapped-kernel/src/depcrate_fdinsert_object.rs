// Generated macro for insert_object (function)
macro_rules! Depcrate_fdinsert_object {
() => {
// Module: crate::fd
// Provides: {"insert_object"}
// Dependencies: {}
pub (crate) fn insert_object (obj : Arc < async_lock :: RwLock < dyn ObjectInterface > > ,) -> io :: Result < FileDescriptor > { core_scheduler () . insert_object (obj) }
};
}
