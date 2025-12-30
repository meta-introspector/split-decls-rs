// Generated macro for binary_search_slice (function)
macro_rules! Depcrate_binary_search_utilbinary_search_slice {
() => {
// Module: crate::binary_search_util
// Provides: {"binary_search_slice"}
// Dependencies: {}
# [doc = " Uses a sorted slice `data: &[E]` as a kind of \"multi-map\". The"] # [doc = " `key_fn` extracts a key of type `K` from the data, and this"] # [doc = " function finds the range of elements that match the key. `data`"] # [doc = " must have been sorted as if by a call to `sort_by_key` for this to"] # [doc = " work."] pub fn binary_search_slice < 'd , E , K > (data : & 'd [E] , key_fn : impl Fn (& E) -> K , key : & K) -> & 'd [E] where K : Ord , { let size = data . len () ; let start = data . partition_point (| x | key_fn (x) < * key) ; if start == size || key_fn (& data [start]) != * key { return & [] ; } ; let offset = start + 1 ; let end = data [offset ..] . partition_point (| x | key_fn (x) <= * key) + offset ; & data [start .. end] }
};
}
