// Generated macro for write_nil (function)
macro_rules! Depcrate_encodewrite_nil {
() => {
// Module: crate::encode
// Provides: {"write_nil"}
// Dependencies: {}
# [doc = " Encodes and attempts to write a nil value into the given write."] # [doc = ""] # [doc = " According to the MessagePack specification, a nil value is represented as a single `0xc0` byte."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `Error` on any I/O error occurred while writing the nil marker."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let mut buf = Vec::new();"] # [doc = ""] # [doc = " rmp::encode::write_nil(&mut buf).unwrap();"] # [doc = ""] # [doc = " assert_eq!(vec![0xc0], buf);"] # [doc = " ```"] # [inline] pub fn write_nil < W : RmpWrite > (wr : & mut W) -> Result < () , W :: Error > { write_marker (wr , Marker :: Null) . map_err (| e | e . 0) }
};
}
