// Generated macro for read_u16 (function)
macro_rules! Depcrate_decode_entriesread_u16 {
() => {
// Module: crate::decode::entries
// Provides: {"read_u16"}
// Dependencies: {}
# [inline] fn read_u16 (data : & [u8]) -> Option < (u16 , & [u8]) > { data . split_at_checked (2) . map (| (num , data) | (u16 :: from_be_bytes (num . try_into () . unwrap ()) , data)) }
};
}
