// Generated macro for impl_29 (impl)
macro_rules! Depcrate_bridge_bufferimpl_29 {
() => {
// Module: crate::bridge::buffer
// Provides: {"impl_29"}
// Dependencies: {}
impl Deref for Buffer { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . data as * const u8 , self . len) } } }
};
}
