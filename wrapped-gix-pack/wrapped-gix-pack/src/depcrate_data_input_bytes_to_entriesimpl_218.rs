// Generated macro for impl_218 (impl)
macro_rules! Depcrate_data_input_bytes_to_entriesimpl_218 {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"impl_218"}
// Dependencies: {}
impl < R > Iterator for BytesToEntriesIter < R > where R : io :: BufRead , { type Item = Result < input :: Entry , input :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . had_error || self . objects_left == 0 { return None ; } let result = self . next_inner () ; self . had_error = result . is_err () ; if self . had_error { self . objects_left = 0 ; } if self . mode == input :: Mode :: Restore && self . had_error { None } else { Some (result) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . objects_left as usize , Some (self . objects_left as usize)) } }
};
}
