// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bioimpl_22 {
() => {
// Module: crate::bio
// Provides: {"impl_22"}
// Dependencies: {}
impl Drop for MemBioSlice < '_ > { fn drop (& mut self) { unsafe { ffi :: BIO_free_all (self . 0) ; } } }
};
}
