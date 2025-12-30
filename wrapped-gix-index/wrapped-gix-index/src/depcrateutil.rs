// Generated macro for util (module)
macro_rules! Depcrateutil {
() => {
// Module: crate
// Provides: {"util"}
// Dependencies: {}
pub (crate) mod util { # [inline] pub fn var_int (data : & [u8]) -> Option < (u64 , & [u8]) > { let (num , consumed) = gix_features :: decode :: leb64_from_read (data) . ok () ? ; let data = & data [consumed ..] ; (num , data) . into () } # [inline] pub fn read_u32 (data : & [u8]) -> Option < (u32 , & [u8]) > { data . split_at_checked (4) . map (| (num , data) | (u32 :: from_be_bytes (num . try_into () . unwrap ()) , data)) } # [inline] pub fn read_u64 (data : & [u8]) -> Option < (u64 , & [u8]) > { data . split_at_checked (8) . map (| (num , data) | (u64 :: from_be_bytes (num . try_into () . unwrap ()) , data)) } # [inline] pub fn from_be_u32 (b : & [u8]) -> u32 { u32 :: from_be_bytes (b . try_into () . unwrap ()) } # [inline] pub fn split_at_byte_exclusive (data : & [u8] , byte : u8) -> Option < (& [u8] , & [u8]) > { if data . len () < 2 { return None ; } data . iter () . enumerate () . find_map (| (idx , b) | { (* b == byte) . then (| | { if idx == 0 { (& [] as & [u8] , & data [1 ..]) } else { let (a , b) = data . split_at (idx) ; (a , & b [1 ..]) } }) }) } }
};
}
