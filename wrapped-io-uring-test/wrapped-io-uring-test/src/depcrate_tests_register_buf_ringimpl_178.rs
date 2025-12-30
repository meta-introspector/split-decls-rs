// Generated macro for impl_178 (impl)
macro_rules! Depcrate_tests_register_buf_ringimpl_178 {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"impl_178"}
// Dependencies: {}
impl Drop for AnonymousMmap { fn drop (& mut self) { unsafe { libc :: munmap (self . addr . as_ptr () , self . len) ; } } }
};
}
