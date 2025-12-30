// Generated macro for impl_46 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_46 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_46"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for EntryValues < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail_index . map (| index | { let entry = self . values . get (index) . unwrap () ; self . tail_index = entry . previous_index ; self . remaining -= 1 ; & entry . value }) } } }
};
}
