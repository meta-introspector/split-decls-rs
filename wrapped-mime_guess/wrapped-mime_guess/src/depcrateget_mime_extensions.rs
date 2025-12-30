// Generated macro for get_mime_extensions (function)
macro_rules! Depcrateget_mime_extensions {
() => {
// Module: crate
// Provides: {"get_mime_extensions"}
// Dependencies: {}
# [doc = " Get a list of known extensions for a given `Mime`."] # [doc = ""] # [doc = " Ignores parameters (only searches with `<main type>/<subtype>`). Case-insensitive (for extension types)."] # [doc = ""] # [doc = " Returns `None` if the MIME type is unknown."] # [doc = ""] # [doc = " ### Wildcards"] # [doc = " If the top-level of the MIME type is a wildcard (`*`), returns all extensions."] # [doc = ""] # [doc = " If the sub-level of the MIME type is a wildcard, returns all extensions for the top-level."] # [cfg (feature = "rev-mappings")] pub fn get_mime_extensions (mime : & Mime) -> Option < & 'static [& 'static str] > { get_extensions (mime . type_ () . as_ref () , mime . subtype () . as_ref ()) }
};
}
