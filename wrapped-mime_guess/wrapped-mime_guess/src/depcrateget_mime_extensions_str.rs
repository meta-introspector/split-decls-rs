// Generated macro for get_mime_extensions_str (function)
macro_rules! Depcrateget_mime_extensions_str {
() => {
// Module: crate
// Provides: {"get_mime_extensions_str"}
// Dependencies: {}
# [doc = " Get a list of known extensions for a MIME type string."] # [doc = ""] # [doc = " Ignores parameters (only searches `<main type>/<subtype>`). Case-insensitive."] # [doc = ""] # [doc = " Returns `None` if the MIME type is unknown."] # [doc = ""] # [doc = " ### Wildcards"] # [doc = " If the top-level of the MIME type is a wildcard (`*`), returns all extensions."] # [doc = ""] # [doc = " If the sub-level of the MIME type is a wildcard, returns all extensions for the top-level."] # [doc = ""] # [doc = " ### Panics"] # [doc = " If `mime_str` is not a valid MIME type specifier (naive)."] # [cfg (feature = "rev-mappings")] pub fn get_mime_extensions_str (mut mime_str : & str) -> Option < & 'static [& 'static str] > { mime_str = mime_str . trim () ; if let Some (sep_idx) = mime_str . find (';') { mime_str = & mime_str [.. sep_idx] ; } let (top , sub) = { let split_idx = mime_str . find ('/') ? ; (& mime_str [.. split_idx] , & mime_str [split_idx + 1 ..]) } ; get_extensions (top , sub) }
};
}
