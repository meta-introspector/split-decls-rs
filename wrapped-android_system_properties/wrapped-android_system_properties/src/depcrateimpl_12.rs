// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Drop for AndroidSystemProperties { fn drop (& mut self) { if ! self . libc_so . is_null () { unsafe { libc :: dlclose (self . libc_so) ; } } } }
};
}
