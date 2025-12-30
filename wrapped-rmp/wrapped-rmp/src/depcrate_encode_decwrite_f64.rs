// Generated macro for write_f64 (function)
macro_rules! Depcrate_encode_decwrite_f64 {
() => {
// Module: crate::encode::dec
// Provides: {"write_f64"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `f64` value as a 9-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the `f64` marker and the others will represent the data itself."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_f64 < W : RmpWrite > (wr : & mut W , val : f64) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: F64) ? ; wr . write_data_f64 (val) ? ; Ok (()) }
};
}
