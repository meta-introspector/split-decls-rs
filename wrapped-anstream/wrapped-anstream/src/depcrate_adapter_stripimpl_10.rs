// Generated macro for impl_10 (impl)
macro_rules! Depcrate_adapter_stripimpl_10 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_10"}
// Dependencies: {}
impl < 's > Iterator for StrippedStr < 's > { type Item = & 's str ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_str (& mut self . bytes , & mut self . state) } }
};
}
