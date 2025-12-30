// Generated macro for parse_header_info (function)
macro_rules! Depcrate_data_entry_decodeparse_header_info {
() => {
// Module: crate::data::entry::decode
// Provides: {"parse_header_info"}
// Dependencies: {}
# [doc = " Parses the header of a pack-entry, yielding object type id, decompressed object size, and consumed bytes"] # [inline] fn parse_header_info (data : & [u8]) -> (u8 , u64 , usize) { let mut c = data [0] ; let mut i = 1 ; let type_id = (c >> 4) & 0b0000_0111 ; let mut size = u64 :: from (c) & 0b0000_1111 ; let mut s = 4 ; while c & 0b1000_0000 != 0 { c = data [i] ; i += 1 ; size += u64 :: from (c & 0b0111_1111) << s ; s += 7 ; } (type_id , size , i) }
};
}
