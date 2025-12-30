// Generated macro for unescape (function)
macro_rules! Depcrate_escapeunescape {
() => {
// Module: crate::escape
// Provides: {"unescape"}
// Dependencies: {}
# [doc = " Unescapes a string."] # [doc = ""] # [doc = " It supports a limited set of escape sequences:"] # [doc = ""] # [doc = " * `\\t`, `\\r` and `\\n` are mapped to their corresponding ASCII bytes."] # [doc = " * `\\xZZ` hexadecimal escapes are mapped to their byte."] # [doc = ""] # [doc = " Everything else is left as is, including non-hexadecimal escapes like"] # [doc = " `\\xGG`."] # [doc = ""] # [doc = " This is useful when it is desirable for a command line argument to be"] # [doc = " capable of specifying arbitrary bytes or otherwise make it easier to"] # [doc = " specify non-printable characters."] # [doc = ""] # [doc = " The dual of this routine is [`escape`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to convert an escaped string (which is valid UTF-8)"] # [doc = " into a corresponding sequence of bytes. Each escape sequence is mapped to"] # [doc = " its bytes, which may include invalid UTF-8."] # [doc = ""] # [doc = " Pay special attention to the use of raw strings. That is, `r\"\\n\"` is"] # [doc = " equivalent to `\"\\\\n\"`."] # [doc = ""] # [doc = " ```"] # [doc = " use grep_cli::unescape;"] # [doc = ""] # [doc = " assert_eq!(&b\"foo\\nbar\\xFFbaz\"[..], &*unescape(r\"foo\\nbar\\xFFbaz\"));"] # [doc = " ```"] pub fn unescape (s : & str) -> Vec < u8 > { Vec :: unescape_bytes (s) }
};
}
