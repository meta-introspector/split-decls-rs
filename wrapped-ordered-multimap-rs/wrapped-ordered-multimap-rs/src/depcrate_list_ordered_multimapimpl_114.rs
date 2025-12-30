// Generated macro for impl_114 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_114 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_114"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for Values < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . next_back () . map (| entry | & entry . value) } }
};
}
