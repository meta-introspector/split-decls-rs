// Generated macro for string_regex_parsed (function)
macro_rules! Depcrate_stringstring_regex_parsed {
() => {
// Module: crate::string
// Provides: {"string_regex_parsed"}
// Dependencies: {}
# [doc = " Like `string_regex()`, but allows providing a pre-parsed expression."] pub fn string_regex_parsed (expr : & Hir) -> ParseResult < String > { bytes_regex_parsed (expr) . map (| v | { v . prop_map (| bytes | { String :: from_utf8 (bytes) . expect ("non-utf8 string") }) . sboxed () }) . map (RegexGeneratorStrategy) }
};
}
