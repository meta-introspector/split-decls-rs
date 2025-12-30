// Generated macro for impl_390 (impl)
macro_rules! Depcrate_bufimpl_390 {
() => {
// Module: crate::buf
// Provides: {"impl_390"}
// Dependencies: {}
impl Deref for Buf { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . raw . ptr as * const u8 , self . raw . size as usize) } } }
};
}
