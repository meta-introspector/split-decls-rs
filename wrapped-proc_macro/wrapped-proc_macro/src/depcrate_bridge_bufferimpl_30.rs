// Generated macro for impl_30 (impl)
macro_rules! Depcrate_bridge_bufferimpl_30 {
() => {
// Module: crate::bridge::buffer
// Provides: {"impl_30"}
// Dependencies: {}
impl DerefMut for Buffer { # [inline] fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self . data , self . len) } } }
};
}
