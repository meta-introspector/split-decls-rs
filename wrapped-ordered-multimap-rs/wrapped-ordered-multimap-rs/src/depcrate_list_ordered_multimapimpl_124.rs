// Generated macro for impl_124 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_124 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_124"}
// Dependencies: {}
impl < 'map , Key , Value > Iterator for ValuesMut < 'map , Key , Value > { type Item = & 'map mut Value ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| entry | & mut entry . value) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
