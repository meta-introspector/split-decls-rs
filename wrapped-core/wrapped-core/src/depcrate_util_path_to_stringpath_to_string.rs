// Generated macro for path_to_string (function)
macro_rules! Depcrate_util_path_to_stringpath_to_string {
() => {
// Module: crate::util::path_to_string
// Provides: {"path_to_string"}
// Dependencies: {}
# [doc = " Transform Rust paths to a readable and comparable string."] # [doc = ""] # [doc = " # Limitations"] # [doc = " * Leading colons are ignored."] # [doc = " * Angle brackets and `as` elements are ignored."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use darling_core::util::path_to_string;"] # [doc = " # use syn::parse_quote;"] # [doc = " assert_eq!(path_to_string(&parse_quote!(a::b)), \"a::b\");"] # [doc = " ```"] pub fn path_to_string (path : & syn :: Path) -> String { path . segments . iter () . map (| s | s . ident . to_string ()) . collect :: < Vec < String > > () . join ("::") }
};
}
