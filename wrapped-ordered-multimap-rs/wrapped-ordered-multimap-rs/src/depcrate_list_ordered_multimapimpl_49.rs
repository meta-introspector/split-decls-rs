// Generated macro for impl_49 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_49 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'map , Key , Value > Iterator for EntryValues < 'map , Key , Value > { type Item = & 'map Value ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head_index . map (| index | { let entry = self . values . get (index) . unwrap () ; self . head_index = entry . next_index ; self . remaining -= 1 ; & entry . value }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
