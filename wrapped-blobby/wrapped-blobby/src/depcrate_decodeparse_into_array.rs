// Generated macro for parse_into_array (function)
macro_rules! Depcrate_decodeparse_into_array {
() => {
// Module: crate::decode
// Provides: {"parse_into_array"}
// Dependencies: {}
# [doc = " Parse blobby data into an array."] pub const fn parse_into_array < const ITEMS_LEN : usize , const DEDUP_LEN : usize > (mut data : & [u8] ,) -> Result < [& [u8] ; ITEMS_LEN] , Error > { match Header :: parse (& mut data) { Ok (header) => { if header . items_len != ITEMS_LEN || header . dedup_len != DEDUP_LEN { return Err (Error :: BadArrayLen) ; } } Err (err) => return Err (err) , } let mut dedup_index : [& [u8] ; DEDUP_LEN] = [& [] ; DEDUP_LEN] ; let mut i = 0 ; while i < dedup_index . len () { let m = try_read_vlq ! (data) ; let split = data . split_at (m) ; dedup_index [i] = split . 0 ; data = split . 1 ; i += 1 ; } let mut res : [& [u8] ; ITEMS_LEN] = [& [] ; ITEMS_LEN] ; let mut i = 0 ; while i < res . len () { let val = try_read_vlq ! (data) ; let is_ref = (val & 1) != 0 ; let val = val >> 1 ; res [i] = if is_ref { if val >= dedup_index . len () { return Err (Error :: InvalidIndex) ; } dedup_index [val] } else { if val > data . len () { return Err (Error :: UnexpectedEnd) ; } let split = data . split_at (val) ; data = split . 1 ; split . 0 } ; i += 1 ; } if data . is_empty () { Ok (res) } else { Err (Error :: BadArrayLen) } }
};
}
