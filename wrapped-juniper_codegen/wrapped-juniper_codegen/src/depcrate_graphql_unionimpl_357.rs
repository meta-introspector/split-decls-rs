// Generated macro for impl_357 (impl)
macro_rules! Depcrate_graphql_unionimpl_357 {
() => {
// Module: crate::graphql_union
// Provides: {"impl_357"}
// Dependencies: {}
impl Attr { # [doc = " Tries to merge two [`Attr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { name : try_merge_opt ! (name : self , another) , description : try_merge_opt ! (description : self , another) , context : try_merge_opt ! (context : self , another) , scalar : try_merge_opt ! (scalar : self , another) , external_resolvers : try_merge_hashmap ! (external_resolvers : self , another => span_joined) , is_internal : self . is_internal || another . is_internal , }) } # [doc = " Parses an [`Attr`] from the provided multiple [`syn::Attribute`]s with"] # [doc = " the specified `names`, placed on a trait or type definition."] fn from_attrs (names : impl AttrNames , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { let mut meta = filter_attrs (names , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) ? ; if meta . description . is_none () { meta . description = Description :: parse_from_doc_attrs (attrs) ? ; } Ok (meta) } }
};
}
