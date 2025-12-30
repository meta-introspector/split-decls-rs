// Generated macro for impl_59 (impl)
macro_rules! Depcrate_secretimpl_59 {
() => {
// Module: crate::secret
// Provides: {"impl_59"}
// Dependencies: {}
impl Secret32 { # [doc = " # Errors"] # [doc = " Returns an error when"] # [doc = " - the string is not 64-bytes long"] # [doc = " - the string contains non-hex characters"] # [doc = ""] # [doc = " The resulting error string does not contain the contents of `s`."] pub fn parse_str (s : & str) -> Result < Secret32 , String > { let bytes = decode_hex (s) . map_err (| e | format ! ("error parsing: {e}")) ? ; Ok (Self (bytes)) } }
};
}
