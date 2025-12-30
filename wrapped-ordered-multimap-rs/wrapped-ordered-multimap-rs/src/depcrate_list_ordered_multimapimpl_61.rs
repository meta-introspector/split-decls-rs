// Generated macro for impl_61 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_61 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_61"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for EntryValuesMut < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail_index . map (| index | { let entry = unsafe { (* self . values) . get_mut (index) } . unwrap () ; self . tail_index = entry . previous_index ; self . remaining -= 1 ; & mut entry . value }) } } }
};
}
