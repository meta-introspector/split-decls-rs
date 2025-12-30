// Generated macro for impl_117 (impl)
macro_rules! Depcrate_os_uniximpl_117 {
() => {
// Module: crate::os::unix
// Provides: {"impl_117"}
// Dependencies: {}
impl < T > Symbol < T > { # [doc = " Convert the loaded `Symbol` into a raw pointer."] pub fn into_raw (self) -> * mut core :: ffi :: c_void { self . pointer } # [doc = " Convert the loaded `Symbol` into a raw pointer."] # [doc = " For unix this does the same as into_raw."] pub fn as_raw_ptr (self) -> * mut core :: ffi :: c_void { self . pointer } }
};
}
