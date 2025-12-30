// Generated macro for encode (function)
macro_rules! Depcrate_data_headerencode {
() => {
// Module: crate::data::header
// Provides: {"encode"}
// Dependencies: {}
# [doc = " Write a pack data header at `version` with `num_objects` and return a buffer."] pub fn encode (version : data :: Version , num_objects : u32) -> [u8 ; 12] { use crate :: data :: Version :: * ; let mut buf = [0u8 ; 12] ; buf [.. 4] . copy_from_slice (b"PACK") ; buf [4 .. 8] . copy_from_slice (& match version { V2 => 2u32 , V3 => 3 , } . to_be_bytes () [..] ,) ; buf [8 ..] . copy_from_slice (& num_objects . to_be_bytes () [..]) ; buf }
};
}
