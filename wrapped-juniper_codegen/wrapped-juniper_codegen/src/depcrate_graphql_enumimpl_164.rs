// Generated macro for impl_164 (impl)
macro_rules! Depcrate_graphql_enumimpl_164 {
() => {
// Module: crate::graphql_enum
// Provides: {"impl_164"}
// Dependencies: {}
impl VariantAttr { # [doc = " Tries to merge two [`VariantAttr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { name : try_merge_opt ! (name : self , another) , description : try_merge_opt ! (description : self , another) , deprecated : try_merge_opt ! (deprecated : self , another) , ignore : try_merge_opt ! (ignore : self , another) , }) } # [doc = " Parses [`VariantAttr`] from the given multiple `name`d"] # [doc = " [`syn::Attribute`]s placed on a trait definition."] fn from_attrs (name : & str , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { let mut attr = filter_attrs (name , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) ? ; if attr . description . is_none () { attr . description = Description :: parse_from_doc_attrs (attrs) ? ; } if attr . deprecated . is_none () { attr . deprecated = deprecation :: Directive :: parse_from_deprecated_attr (attrs) ? ; } Ok (attr) } }
};
}
