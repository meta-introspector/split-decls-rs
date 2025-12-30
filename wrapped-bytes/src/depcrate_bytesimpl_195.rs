// Generated macro for impl_195 (impl)
macro_rules! Depcrate_bytesimpl_195 {
() => {
// Module: crate::bytes
// Provides: {"impl_195"}
// Dependencies: {}
impl Drop for Shared { fn drop (& mut self) { unsafe { dealloc (self . buf , Layout :: from_size_align (self . cap , 1) . unwrap ()) } } }
};
}
