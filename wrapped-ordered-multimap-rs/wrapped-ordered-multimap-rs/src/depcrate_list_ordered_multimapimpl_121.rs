// Generated macro for impl_121 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_121 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_121"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for ValuesMut < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . next_back () . map (| entry | & mut entry . value) } }
};
}
