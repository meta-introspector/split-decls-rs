// Generated macro for impl_55 (impl)
macro_rules! Depcrate_secretimpl_55 {
() => {
// Module: crate::secret
// Provides: {"impl_55"}
// Dependencies: {}
impl Secret16 { # [doc = " # Errors"] # [doc = " Returns an error when"] # [doc = " - the string is not 32-bytes long"] # [doc = " - the string contains non-hex characters"] # [doc = ""] # [doc = " The resulting error string does not contain the contents of `s`."] pub fn parse_str (s : & str) -> Result < Secret16 , String > { let bytes = decode_hex (s) . map_err (| e | format ! ("error parsing: {e}")) ? ; Ok (Self (bytes)) } }
};
}
