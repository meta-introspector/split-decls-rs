// Generated macro for impl_115 (impl)
macro_rules! Depcrate_zlibimpl_115 {
() => {
// Module: crate::zlib
// Provides: {"impl_115"}
// Dependencies: {}
impl Drop for Decompress { fn drop (& mut self) { unsafe { libz_rs_sys :: inflateEnd (& mut self . 0) } ; } }
};
}
