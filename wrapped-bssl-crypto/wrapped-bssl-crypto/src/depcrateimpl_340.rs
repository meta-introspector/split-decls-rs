// Generated macro for impl_340 (impl)
macro_rules! Depcrateimpl_340 {
() => {
// Module: crate
// Provides: {"impl_340"}
// Dependencies: {}
impl Buffer { # [doc = " Safety: `ptr` must point to `len` bytes, allocated by BoringSSL."] unsafe fn new (ptr : * mut u8 , len : usize) -> Buffer { Buffer { ptr , len } } }
};
}
