// Generated macro for impl_19 (impl)
macro_rules! Depcrate_listimpl_19 {
() => {
// Module: crate::list
// Provides: {"impl_19"}
// Dependencies: {}
impl < T > DoubleEndedIterator for FixedSizeListIter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . len > 0 { let back = self . back ; let node = self . list . node_ref (back) . unwrap () ; self . back = node . prev ; self . len -= 1 ; Some ((back , & node . data)) } else { None } } }
};
}
