// Generated macro for impl_12 (impl)
macro_rules! Depcrate_adt_dagimpl_12 {
() => {
// Module: crate::adt::dag
// Provides: {"impl_12"}
// Dependencies: {}
impl Iterator for NodeIterator { type Item = NodeHandle ; fn next (& mut self) -> Option < Self :: Item > { if self . curr == self . last { return None ; } let item = Some (NodeHandle :: from (self . curr)) ; self . curr += 1 ; item } }
};
}
