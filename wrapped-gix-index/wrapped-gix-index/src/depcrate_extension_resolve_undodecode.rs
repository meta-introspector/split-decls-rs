// Generated macro for decode (function)
macro_rules! Depcrate_extension_resolve_undodecode {
() => {
// Module: crate::extension::resolve_undo
// Provides: {"decode"}
// Dependencies: {}
pub fn decode (mut data : & [u8] , object_hash : gix_hash :: Kind) -> Option < Paths > { let hash_len = object_hash . len_in_bytes () ; let mut out = Vec :: new () ; while ! data . is_empty () { let (path , rest) = split_at_byte_exclusive (data , 0) ? ; data = rest ; let mut modes = [0u32 ; 3] ; for mode in & mut modes { let (mode_ascii , rest) = split_at_byte_exclusive (data , 0) ? ; data = rest ; * mode = u32 :: from_str_radix (std :: str :: from_utf8 (mode_ascii) . ok () ? , 8) . ok () ? ; } let mut stages = [None , None , None] ; for (mode , stage) in modes . iter () . zip (stages . iter_mut ()) { if * mode == 0 { continue ; } let (hash , rest) = data . split_at_checked (hash_len) ? ; data = rest ; * stage = Some (Stage { mode : * mode , id : ObjectId :: from_bytes_or_panic (hash) , }) ; } out . push (ResolvePath { name : path . into () , stages , }) ; } out . into () }
};
}
