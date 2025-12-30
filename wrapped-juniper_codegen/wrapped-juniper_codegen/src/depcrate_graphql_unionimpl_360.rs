// Generated macro for impl_360 (impl)
macro_rules! Depcrate_graphql_unionimpl_360 {
() => {
// Module: crate::graphql_union
// Provides: {"impl_360"}
// Dependencies: {}
impl VariantAttr { # [doc = " Tries to merge two [`VariantAttr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { ignore : try_merge_opt ! (ignore : self , another) , external_resolver : try_merge_opt ! (external_resolver : self , another) , }) } # [doc = " Parses [`VariantAttr`] from the given multiple `name`d"] # [doc = " [`syn::Attribute`]s placed on a variant/field/method definition."] fn from_attrs (name : & str , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { filter_attrs (name , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) } }
};
}
