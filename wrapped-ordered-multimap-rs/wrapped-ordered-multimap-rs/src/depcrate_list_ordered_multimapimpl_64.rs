// Generated macro for impl_64 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_64 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'map , Key , Value > Iterator for EntryValuesMut < 'map , Key , Value > { type Item = & 'map mut Value ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head_index . map (| index | { let entry = unsafe { (* self . values) . get_mut (index) } . unwrap () ; self . head_index = entry . next_index ; self . remaining -= 1 ; & mut entry . value }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
