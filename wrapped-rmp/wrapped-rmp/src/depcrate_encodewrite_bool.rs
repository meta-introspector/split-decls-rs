// Generated macro for write_bool (function)
macro_rules! Depcrate_encodewrite_bool {
() => {
// Module: crate::encode
// Provides: {"write_bool"}
// Dependencies: {}
# [doc = " Encodes and attempts to write a bool value into the given write."] # [doc = ""] # [doc = " According to the MessagePack specification, an encoded boolean value is represented as a single"] # [doc = " byte."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Each call to this function may generate an I/O error indicating that the operation could not be"] # [doc = " completed."] # [inline] pub fn write_bool < W : RmpWrite > (wr : & mut W , val : bool) -> Result < () , W :: Error > { let marker = if val { Marker :: True } else { Marker :: False } ; write_marker (wr , marker) . map_err (| e | e . 0) }
};
}
