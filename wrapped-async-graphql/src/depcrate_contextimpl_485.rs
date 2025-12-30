// Generated macro for impl_485 (impl)
macro_rules! Depcrate_contextimpl_485 {
() => {
// Module: crate::context
// Provides: {"impl_485"}
// Dependencies: {}
impl < 'a > Iterator for Parents < 'a > { type Item = & 'a QueryPathNode < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let parent = self . 0 . parent ; if let Some (parent) = parent { self . 0 = parent ; } parent } }
};
}
