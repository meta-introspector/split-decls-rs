// Generated macro for impl_38 (impl)
macro_rules! Depcrate_digestimpl_38 {
() => {
// Module: crate::digest
// Provides: {"impl_38"}
// Dependencies: {}
impl Digest16 { # [doc = " # Errors"] # [doc = " Returns an error when"] # [doc = " - it fails to parse the string in hex format"] # [doc = " - the string is not 64-bytes long"] # [doc = ""] # [doc = " The resulting error string does not contain the contents of `s`."] pub fn parse_str (s : & str) -> Result < Digest16 , String > { Ok (Self (decode_hex (s) . map_err (| e | format ! ("error parsing digest: {e}")) ? ,)) } }
};
}
