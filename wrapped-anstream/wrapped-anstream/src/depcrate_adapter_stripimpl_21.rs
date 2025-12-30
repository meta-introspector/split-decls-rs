// Generated macro for impl_21 (impl)
macro_rules! Depcrate_adapter_stripimpl_21 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_21"}
// Dependencies: {}
impl < 's > Iterator for StrippedBytes < 's > { type Item = & 's [u8] ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_bytes (& mut self . bytes , & mut self . state , & mut self . utf8parser) } }
};
}
