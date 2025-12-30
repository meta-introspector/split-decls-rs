// Generated macro for get_extensions (function)
macro_rules! Depcrateget_extensions {
() => {
// Module: crate
// Provides: {"get_extensions"}
// Dependencies: {}
# [doc = " Get the extensions for a given top-level and sub-level of a MIME type"] # [doc = " (`{toplevel}/{sublevel}`)."] # [doc = ""] # [doc = " Returns `None` if `toplevel` or `sublevel` are unknown."] # [doc = ""] # [doc = " ### Wildcards"] # [doc = " If the top-level of the MIME type is a wildcard (`*`), returns all extensions."] # [doc = ""] # [doc = " If the sub-level of the MIME type is a wildcard, returns all extensions for the top-level."] # [cfg (feature = "rev-mappings")] pub fn get_extensions (toplevel : & str , sublevel : & str) -> Option < & 'static [& 'static str] > { impl_ :: get_extensions (toplevel , sublevel) }
};
}
