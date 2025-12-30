// Generated macro for impl_25 (impl)
macro_rules! Depcrate_bioimpl_25 {
() => {
// Module: crate::bio
// Provides: {"impl_25"}
// Dependencies: {}
impl Drop for MemBio { fn drop (& mut self) { unsafe { ffi :: BIO_free_all (self . 0) ; } } }
};
}
