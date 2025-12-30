// Generated macro for impl_53 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_53 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_53"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for EntryValuesDrain < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail_index . map (| index | { let entry = self . values . remove (index) . unwrap () ; self . tail_index = entry . previous_index ; self . remaining -= 1 ; entry . value }) } } }
};
}
