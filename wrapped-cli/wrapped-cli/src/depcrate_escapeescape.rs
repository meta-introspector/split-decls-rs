// Generated macro for escape (function)
macro_rules! Depcrate_escapeescape {
() => {
// Module: crate::escape
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Escapes arbitrary bytes into a human readable string."] # [doc = ""] # [doc = " This converts `\\t`, `\\r` and `\\n` into their escaped forms. It also"] # [doc = " converts the non-printable subset of ASCII in addition to invalid UTF-8"] # [doc = " bytes to hexadecimal escape sequences. Everything else is left as is."] # [doc = ""] # [doc = " The dual of this routine is [`unescape`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to convert a byte string that contains a `\\n` and"] # [doc = " invalid UTF-8 bytes into a `String`."] # [doc = ""] # [doc = " Pay special attention to the use of raw strings. That is, `r\"\\n\"` is"] # [doc = " equivalent to `\"\\\\n\"`."] # [doc = ""] # [doc = " ```"] # [doc = " use grep_cli::escape;"] # [doc = ""] # [doc = " assert_eq!(r\"foo\\nbar\\xFFbaz\", escape(b\"foo\\nbar\\xFFbaz\"));"] # [doc = " ```"] pub fn escape (bytes : & [u8]) -> String { bytes . escape_bytes () . to_string () }
};
}
