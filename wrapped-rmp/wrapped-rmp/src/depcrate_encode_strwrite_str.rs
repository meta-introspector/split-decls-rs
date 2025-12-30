// Generated macro for write_str (function)
macro_rules! Depcrate_encode_strwrite_str {
() => {
// Module: crate::encode::str
// Provides: {"write_str"}
// Dependencies: {}
# [doc = " Encodes and attempts to write the most efficient string binary representation to the"] # [doc = " given `Write`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_str < W : RmpWrite > (wr : & mut W , data : & str) -> Result < () , ValueWriteError < W :: Error > > { write_str_len (wr , data . len () as u32) ? ; wr . write_bytes (data . as_bytes ()) . map_err (ValueWriteError :: InvalidDataWrite) }
};
}
