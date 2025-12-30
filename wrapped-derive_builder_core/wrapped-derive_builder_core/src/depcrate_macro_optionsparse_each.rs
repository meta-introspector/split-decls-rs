// Generated macro for parse_each (function)
macro_rules! Depcrate_macro_optionsparse_each {
() => {
// Module: crate::macro_options
// Provides: {"parse_each"}
// Dependencies: {}
# [doc = " Create `Each` from an attribute's `Meta`."] # [doc = ""] # [doc = " Two formats are supported:"] # [doc = ""] # [doc = " * `each = \"...\"`, which provides the name of the `each` setter and otherwise uses default values"] # [doc = " * `each(name = \"...\")`, which allows setting additional options on the `each` setter"] fn parse_each (meta : & Meta) -> darling :: Result < Option < Each > > { if let Meta :: NameValue (mnv) = meta { Ident :: from_meta (meta) . map (Each :: from) . map (Some) . map_err (| e | e . with_span (& mnv . value)) } else { Each :: from_meta (meta) . map (Some) } }
};
}
