// Generated macro for impl_14 (impl)
macro_rules! Depcrate_adapter_stripimpl_14 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_14"}
// Dependencies: {}
impl < 's > Iterator for StripStrIter < 's > { type Item = & 's str ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_str (& mut self . bytes , self . state) } }
};
}
