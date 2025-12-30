// Generated macro for skip_search (function)
macro_rules! Depcrate_unicode_dataskip_search {
() => {
// Module: crate::unicode_data
// Provides: {"skip_search"}
// Dependencies: {}
# [inline (always)] fn skip_search < const SOR : usize , const OFFSETS : usize > (needle : u32 , short_offset_runs : & [u32 ; SOR] , offsets : & [u8 ; OFFSETS] ,) -> bool { let last_idx = match short_offset_runs . binary_search_by_key (& (needle << 11) , | header | header << 11) { Ok (idx) => idx + 1 , Err (idx) => idx , } ; let mut offset_idx = decode_length (short_offset_runs [last_idx]) ; let length = if let Some (next) = short_offset_runs . get (last_idx + 1) { decode_length (* next) - offset_idx } else { offsets . len () - offset_idx } ; let prev = last_idx . checked_sub (1) . map (| prev | decode_prefix_sum (short_offset_runs [prev])) . unwrap_or (0) ; let total = needle - prev ; let mut prefix_sum = 0 ; for _ in 0 .. (length - 1) { let offset = offsets [offset_idx] ; prefix_sum += offset as u32 ; if prefix_sum > total { break ; } offset_idx += 1 ; } offset_idx % 2 == 1 }
};
}
