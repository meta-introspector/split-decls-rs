// Generated macro for impl_57 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_57 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_57"}
// Dependencies: {}
impl < Key , Value > Iterator for EntryValuesDrain < '_ , Key , Value > { type Item = Value ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head_index . map (| index | { let entry = self . values . remove (index) . unwrap () ; self . head_index = entry . next_index ; self . remaining -= 1 ; entry . value }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
