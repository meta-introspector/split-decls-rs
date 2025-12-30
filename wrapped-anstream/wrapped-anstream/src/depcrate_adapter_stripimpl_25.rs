// Generated macro for impl_25 (impl)
macro_rules! Depcrate_adapter_stripimpl_25 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_25"}
// Dependencies: {}
impl < 's > Iterator for StripBytesIter < 's > { type Item = & 's [u8] ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_bytes (& mut self . bytes , self . state , self . utf8parser) } }
};
}
