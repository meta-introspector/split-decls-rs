// Generated macro for streaming_parse_header_info (function)
macro_rules! Depcrate_data_entry_decodestreaming_parse_header_info {
() => {
// Module: crate::data::entry::decode
// Provides: {"streaming_parse_header_info"}
// Dependencies: {}
# [inline] fn streaming_parse_header_info (read : & mut dyn io :: Read) -> Result < (u8 , u64 , usize) , io :: Error > { let mut byte = [0u8 ; 1] ; read . read_exact (& mut byte) ? ; let mut c = byte [0] ; let mut i = 1 ; let type_id = (c >> 4) & 0b0000_0111 ; let mut size = u64 :: from (c) & 0b0000_1111 ; let mut s = 4 ; while c & 0b1000_0000 != 0 { read . read_exact (& mut byte) ? ; c = byte [0] ; i += 1 ; size += u64 :: from (c & 0b0111_1111) << s ; s += 7 ; } Ok ((type_id , size , i)) }
};
}
