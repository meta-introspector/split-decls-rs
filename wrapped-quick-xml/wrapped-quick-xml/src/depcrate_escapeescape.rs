// Generated macro for escape (function)
macro_rules! Depcrate_escapeescape {
() => {
// Module: crate::escape
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Escapes an `&str` and replaces all xml special characters (`<`, `>`, `&`, `'`, `\"`)"] # [doc = " with their corresponding xml escaped value."] # [doc = ""] # [doc = " This function performs following replacements:"] # [doc = ""] # [doc = " | Character | Replacement"] # [doc = " |-----------|------------"] # [doc = " | `<`       | `&lt;`"] # [doc = " | `>`       | `&gt;`"] # [doc = " | `&`       | `&amp;`"] # [doc = " | `'`       | `&apos;`"] # [doc = " | `\"`       | `&quot;`"] # [doc = ""] # [doc = " This function performs following replacements:"] # [doc = ""] # [doc = " | Character | Replacement"] # [doc = " |-----------|------------"] # [doc = " | `<`       | `&lt;`"] # [doc = " | `>`       | `&gt;`"] # [doc = " | `&`       | `&amp;`"] # [doc = " | `'`       | `&apos;`"] # [doc = " | `\"`       | `&quot;`"] pub fn escape < 'a > (raw : impl Into < Cow < 'a , str > >) -> Cow < 'a , str > { _escape (raw , | ch | matches ! (ch , b'<' | b'>' | b'&' | b'\'' | b'\"')) }
};
}
