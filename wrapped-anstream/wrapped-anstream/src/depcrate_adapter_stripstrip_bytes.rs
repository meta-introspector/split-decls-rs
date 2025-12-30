// Generated macro for strip_bytes (function)
macro_rules! Depcrate_adapter_stripstrip_bytes {
() => {
// Module: crate::adapter::strip
// Provides: {"strip_bytes"}
// Dependencies: {}
# [doc = " Strip ANSI escapes from bytes, returning the printable content"] # [doc = ""] # [doc = " This can be used to take output from a program that includes escape sequences and write it"] # [doc = " somewhere that does not easily support them, such as a log file."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::io::Write as _;"] # [doc = ""] # [doc = " let styled_text = \"\\x1b[32mfoo\\x1b[m bar\";"] # [doc = " let plain_str = anstream::adapter::strip_bytes(styled_text.as_bytes()).into_vec();"] # [doc = " assert_eq!(plain_str.as_slice(), &b\"foo bar\"[..]);"] # [doc = " ```"] # [inline] pub fn strip_bytes (data : & [u8]) -> StrippedBytes < '_ > { StrippedBytes :: new (data) }
};
}
