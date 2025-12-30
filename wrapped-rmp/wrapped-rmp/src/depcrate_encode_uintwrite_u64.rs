// Generated macro for write_u64 (function)
macro_rules! Depcrate_encode_uintwrite_u64 {
() => {
// Module: crate::encode::uint
// Provides: {"write_u64"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `u64` value strictly as a 9-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the marker and the others will represent the data itself."] # [doc = ""] # [doc = " Note, that this function will encode the given value in 9-byte sequence no matter what, even if"] # [doc = " the value can be represented using single byte as a positive fixnum."] # [doc = ""] # [doc = " If you need to fit the given buffer efficiently use `write_uint` instead, which automatically"] # [doc = " selects the appropriate integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_u64 < W : RmpWrite > (wr : & mut W , val : u64) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: U64) ? ; wr . write_data_u64 (val) ? ; Ok (()) }
};
}
