// Generated macro for parse_graphql_attrs (function)
macro_rules! Depcrate_utilsparse_graphql_attrs {
() => {
// Module: crate::utils
// Provides: {"parse_graphql_attrs"}
// Dependencies: {}
pub fn parse_graphql_attrs < T : FromMeta + Default > (attrs : & [Attribute] ,) -> GeneratorResult < Option < T > > { for attr in attrs { if attr . path () . is_ident ("graphql") { return Ok (Some (T :: from_meta (& attr . meta) ?)) ; } } Ok (None) }
};
}
