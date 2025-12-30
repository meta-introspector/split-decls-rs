// Generated macro for index_entries_sorted_by_offset_ascending (function)
macro_rules! Depcrate_index_utilindex_entries_sorted_by_offset_ascending {
() => {
// Module: crate::index::util
// Provides: {"index_entries_sorted_by_offset_ascending"}
// Dependencies: {}
pub (crate) fn index_entries_sorted_by_offset_ascending (idx : & crate :: index :: File , progress : & mut dyn Progress ,) -> Vec < crate :: index :: Entry > { progress . init (Some (idx . num_objects as usize) , progress :: count ("entries")) ; let start = Instant :: now () ; let mut v = exact_vec (idx . num_objects as usize) ; for entry in idx . iter () { v . push (entry) ; progress . inc () ; } v . sort_by_key (| e | e . pack_offset) ; progress . show_throughput (start) ; v }
};
}
