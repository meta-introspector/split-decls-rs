// Generated macro for impl_459 (impl)
macro_rules! Depcrate_traitsimpl_459 {
() => {
// Module: crate::traits
// Provides: {"impl_459"}
// Dependencies: {}
impl Iterator for SaturatingIterator { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { let old_count = self . count ; self . count = self . count . saturating_add (1) ; Some (old_count) } }
};
}
