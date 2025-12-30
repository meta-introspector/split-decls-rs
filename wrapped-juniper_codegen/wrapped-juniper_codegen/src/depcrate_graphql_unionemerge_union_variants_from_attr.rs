// Generated macro for emerge_union_variants_from_attr (function)
macro_rules! Depcrate_graphql_unionemerge_union_variants_from_attr {
() => {
// Module: crate::graphql_union
// Provides: {"emerge_union_variants_from_attr"}
// Dependencies: {}
# [doc = " Emerges [`Attr::external_resolvers`] into the given [GraphQL union][1]"] # [doc = " `variants`."] # [doc = ""] # [doc = " If duplication happens, then resolving code is overwritten with the one from"] # [doc = " `external_resolvers`."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] fn emerge_union_variants_from_attr (variants : & mut Vec < VariantDefinition > , external_resolvers : AttrResolvers ,) { if external_resolvers . is_empty () { return ; } for (ty , rslvr) in external_resolvers { let resolver_fn = rslvr . into_inner () ; let resolver_code = parse_quote ! { # resolver_fn (self , :: juniper :: FromContext :: from (context)) } ; let resolver_check = parse_quote ! { ({ # resolver_code } as :: core :: option :: Option <&# ty >) . is_some () } ; if let Some (var) = variants . iter_mut () . find (| v | v . ty == ty) { var . resolver_code = resolver_code ; var . resolver_check = resolver_check ; } else { variants . push (VariantDefinition { ty , resolver_code , resolver_check , context : None , }) } } }
};
}
