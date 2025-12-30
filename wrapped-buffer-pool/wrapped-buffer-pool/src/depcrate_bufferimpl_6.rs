// Generated macro for impl_6 (impl)
macro_rules! Depcrate_bufferimpl_6 {
() => {
// Module: crate::buffer
// Provides: {"impl_6"}
// Dependencies: {}
impl Deref for ConsumeBuffer { type Target = [u8] ; fn deref (& self) -> & Self :: Target { & self . inner [self . head ..] } }
};
}
