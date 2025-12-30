// Generated macro for minimal_escape (function)
macro_rules! Depcrate_escapeminimal_escape {
() => {
// Module: crate::escape
// Provides: {"minimal_escape"}
// Dependencies: {}
# [doc = " XML standard [requires] that only `<` and `&` was escaped in text content or"] # [doc = " attribute value. All other characters not necessary to be escaped, although"] # [doc = " for compatibility with SGML they also should be escaped. Practically, escaping"] # [doc = " only those characters is enough."] # [doc = ""] # [doc = " This function performs following replacements:"] # [doc = ""] # [doc = " | Character | Replacement"] # [doc = " |-----------|------------"] # [doc = " | `<`       | `&lt;`"] # [doc = " | `&`       | `&amp;`"] # [doc = ""] # [doc = " [requires]: https://www.w3.org/TR/xml11/#syntax"] pub fn minimal_escape < 'a > (raw : impl Into < Cow < 'a , str > >) -> Cow < 'a , str > { _escape (raw , | ch | matches ! (ch , b'<' | b'&')) }
};
}
