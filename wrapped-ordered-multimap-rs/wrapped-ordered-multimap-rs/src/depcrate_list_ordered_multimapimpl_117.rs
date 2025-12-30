// Generated macro for impl_117 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_117 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'map , Key , Value > Iterator for Values < 'map , Key , Value > { type Item = & 'map Value ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| entry | & entry . value) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
