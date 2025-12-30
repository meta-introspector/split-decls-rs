// Generated macro for partial_escape (function)
macro_rules! Depcrate_escapepartial_escape {
() => {
// Module: crate::escape
// Provides: {"partial_escape"}
// Dependencies: {}
# [doc = " Escapes an `&str` and replaces xml special characters (`<`, `>`, `&`)"] # [doc = " with their corresponding xml escaped value."] # [doc = ""] # [doc = " Should only be used for escaping text content. In XML text content, it is allowed"] # [doc = " (though not recommended) to leave the quote special characters `\"` and `'` unescaped."] # [doc = ""] # [doc = " This function performs following replacements:"] # [doc = ""] # [doc = " | Character | Replacement"] # [doc = " |-----------|------------"] # [doc = " | `<`       | `&lt;`"] # [doc = " | `>`       | `&gt;`"] # [doc = " | `&`       | `&amp;`"] # [doc = ""] # [doc = " This function performs following replacements:"] # [doc = ""] # [doc = " | Character | Replacement"] # [doc = " |-----------|------------"] # [doc = " | `<`       | `&lt;`"] # [doc = " | `>`       | `&gt;`"] # [doc = " | `&`       | `&amp;`"] pub fn partial_escape < 'a > (raw : impl Into < Cow < 'a , str > >) -> Cow < 'a , str > { _escape (raw , | ch | matches ! (ch , b'<' | b'>' | b'&')) }
};
}
