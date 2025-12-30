// Generated macro for write_i16 (function)
macro_rules! Depcrate_encode_sintwrite_i16 {
() => {
// Module: crate::encode::sint
// Provides: {"write_i16"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `i16` value as a 3-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the marker and the others will represent the data itself."] # [doc = ""] # [doc = " Note, that this function will encode the given value in 3-byte sequence no matter what, even if"] # [doc = " the value can be represented using single byte as a fixnum. Also note, that the first byte will"] # [doc = " always be the i16 marker (`0xd1`)."] # [doc = ""] # [doc = " If you need to fit the given buffer efficiently use `write_sint` instead, which automatically"] # [doc = " selects the appropriate integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_i16 < W : RmpWrite > (wr : & mut W , val : i16) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: I16) ? ; wr . write_data_i16 (val) ? ; Ok (()) }
};
}
