// Generated macro for impl_391 (impl)
macro_rules! Depcrate_bufimpl_391 {
() => {
// Module: crate::buf
// Provides: {"impl_391"}
// Dependencies: {}
impl DerefMut for Buf { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self . raw . ptr as * mut u8 , self . raw . size as usize) } } }
};
}
