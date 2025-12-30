// Generated macro for unescape (function)
macro_rules! Depcrate_escapeunescape {
() => {
// Module: crate::escape
// Provides: {"unescape"}
// Dependencies: {}
# [doc = " Unescape an `&str` and replaces all xml escaped characters (`&...;`) into"] # [doc = " their corresponding value."] # [doc = ""] # [doc = " If feature [`escape-html`] is enabled, then recognizes all [HTML5 escapes]."] # [doc = ""] # [doc = " [`escape-html`]: ../index.html#escape-html"] # [doc = " [HTML5 escapes]: https://dev.w3.org/html5/html-author/charref"] pub fn unescape (raw : & str) -> Result < Cow < '_ , str > , EscapeError > { unescape_with (raw , resolve_predefined_entity) }
};
}
