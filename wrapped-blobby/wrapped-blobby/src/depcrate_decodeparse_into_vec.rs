// Generated macro for parse_into_vec (function)
macro_rules! Depcrate_decodeparse_into_vec {
() => {
// Module: crate::decode
// Provides: {"parse_into_vec"}
// Dependencies: {}
# [doc = " Parse blobby data into a vector of slices."] # [cfg (feature = "alloc")] pub fn parse_into_vec (mut data : & [u8]) -> Result < alloc :: vec :: Vec < & [u8] > , Error > { use alloc :: { vec , vec :: Vec } ; let Header { items_len , dedup_len , } = Header :: parse (& mut data) ? ; let mut dedup_index : Vec < & [u8] > = vec ! [& [] ; dedup_len] ; let mut i = 0 ; while i < dedup_index . len () { let m = try_read_vlq ! (data) ; let split = data . split_at (m) ; dedup_index [i] = split . 0 ; data = split . 1 ; i += 1 ; } let mut res : Vec < & [u8] > = vec ! [& [] ; items_len] ; let mut i = 0 ; while i < res . len () { let val = try_read_vlq ! (data) ; let is_ref = (val & 1) != 0 ; let val = val >> 1 ; res [i] = if is_ref { if val >= dedup_index . len () { return Err (Error :: InvalidIndex) ; } dedup_index [val] } else { if val > data . len () { return Err (Error :: UnexpectedEnd) ; } let split = data . split_at (val) ; data = split . 1 ; split . 0 } ; i += 1 ; } assert ! (data . is_empty ()) ; Ok (res) }
};
}
