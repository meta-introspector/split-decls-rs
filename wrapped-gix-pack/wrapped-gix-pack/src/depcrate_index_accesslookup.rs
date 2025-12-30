// Generated macro for lookup (function)
macro_rules! Depcrate_index_accesslookup {
() => {
// Module: crate::index::access
// Provides: {"lookup"}
// Dependencies: {}
pub (crate) fn lookup < 'a > (id : & gix_hash :: oid , fan : & [u32 ; FAN_LEN] , oid_at_index : & dyn Fn (EntryIndex) -> & 'a gix_hash :: oid ,) -> Option < EntryIndex > { let first_byte = id . first_byte () as usize ; let mut upper_bound = fan [first_byte] ; let mut lower_bound = if first_byte != 0 { fan [first_byte - 1] } else { 0 } ; while lower_bound < upper_bound { let mid = (lower_bound + upper_bound) / 2 ; let mid_sha = oid_at_index (mid) ; use std :: cmp :: Ordering :: * ; match id . cmp (mid_sha) { Less => upper_bound = mid , Equal => return Some (mid) , Greater => lower_bound = mid + 1 , } } None }
};
}
