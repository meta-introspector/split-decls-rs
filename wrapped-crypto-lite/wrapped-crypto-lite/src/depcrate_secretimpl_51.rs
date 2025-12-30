// Generated macro for impl_51 (impl)
macro_rules! Depcrate_secretimpl_51 {
() => {
// Module: crate::secret
// Provides: {"impl_51"}
// Dependencies: {}
impl Secret12 { # [doc = " # Errors"] # [doc = " Returns an error when"] # [doc = " - the string is not 24-bytes long"] # [doc = " - the string contains non-hex characters"] # [doc = ""] # [doc = " The resulting error string does not contain the contents of `s`."] pub fn parse_str (s : & str) -> Result < Secret12 , String > { let bytes = decode_hex (s) . map_err (| e | format ! ("error parsing: {e}")) ? ; Ok (Self (bytes)) } }
};
}
