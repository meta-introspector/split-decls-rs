// Generated macro for parse_attribute_to_meta_list (function)
macro_rules! Depcrate_util_parse_attributeparse_attribute_to_meta_list {
() => {
// Module: crate::util::parse_attribute
// Provides: {"parse_attribute_to_meta_list"}
// Dependencies: {}
# [doc = " Try to parse an attribute into a meta list. Path-type meta values are accepted and returned"] # [doc = " as empty lists with their passed-in path. Name-value meta values and non-meta attributes"] # [doc = " will cause errors to be returned."] pub fn parse_attribute_to_meta_list (attr : & Attribute) -> Result < MetaList > { match & attr . meta { Meta :: List (list) => Ok (list . clone ()) , Meta :: NameValue (nv) => Err (Error :: custom (format ! ("Name-value arguments are not supported. Use #[{}(...)]" , DisplayPath (& nv . path))) . with_span (& nv)) , Meta :: Path (path) => Ok (MetaList { path : path . clone () , delimiter : syn :: MacroDelimiter :: Paren (token :: Paren { span : { let mut group = proc_macro2 :: Group :: new (proc_macro2 :: Delimiter :: None , proc_macro2 :: TokenStream :: new () ,) ; group . set_span (attr . span ()) ; group . delim_span () } , }) , tokens : Default :: default () , }) , } }
};
}
