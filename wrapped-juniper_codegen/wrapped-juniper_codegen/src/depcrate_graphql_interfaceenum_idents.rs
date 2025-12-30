// Generated macro for enum_idents (function)
macro_rules! Depcrate_graphql_interfaceenum_idents {
() => {
// Module: crate::graphql_interface
// Provides: {"enum_idents"}
// Dependencies: {}
# [doc = " Returns [`syn::Ident`]s for a generic enum deriving [`Clone`] and [`Copy`]"] # [doc = " on it and enum alias which generic arguments are filled with"] # [doc = " [GraphQL interface][1] implementers."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Interfaces"] fn enum_idents (trait_ident : & syn :: Ident , alias_ident : Option < & syn :: Ident > ,) -> (syn :: Ident , syn :: Ident) { let enum_alias_ident = alias_ident . cloned () . unwrap_or_else (| | format_ident ! ("{trait_ident}Value")) ; let enum_ident = alias_ident . map_or_else (| | format_ident ! ("{trait_ident}ValueEnum") , | c | format_ident ! ("{c}Enum") ,) ; (enum_ident , enum_alias_ident) }
};
}
