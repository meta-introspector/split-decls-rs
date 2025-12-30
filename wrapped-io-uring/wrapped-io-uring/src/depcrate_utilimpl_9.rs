// Generated macro for impl_9 (impl)
macro_rules! Depcrate_utilimpl_9 {
() => {
// Module: crate::util
// Provides: {"impl_9"}
// Dependencies: {}
impl Drop for Mmap { fn drop (& mut self) { unsafe { libc :: munmap (self . addr . as_ptr () , self . len) ; } } }
};
}
