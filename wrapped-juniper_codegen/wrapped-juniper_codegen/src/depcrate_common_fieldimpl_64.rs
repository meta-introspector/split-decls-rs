// Generated macro for impl_64 (impl)
macro_rules! Depcrate_common_fieldimpl_64 {
() => {
// Module: crate::common::field
// Provides: {"impl_64"}
// Dependencies: {}
impl Attr { # [doc = " Tries to merge two [`Attrs`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { name : try_merge_opt ! (name : self , another) , description : try_merge_opt ! (description : self , another) , deprecated : try_merge_opt ! (deprecated : self , another) , ignore : try_merge_opt ! (ignore : self , another) , }) } # [doc = " Parses [`Attr`] from the given multiple `name`d [`syn::Attribute`]s"] # [doc = " placed on a [GraphQL field][1] definition."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Language.Fields"] pub (crate) fn from_attrs (name : & str , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { let mut attr = filter_attrs (name , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) ? ; if let Some (ignore) = & attr . ignore { if attr . name . is_some () || attr . description . is_some () || attr . deprecated . is_some () { return Err (syn :: Error :: new (ignore . span () , "`ignore` attribute argument is not composable with any other arguments" ,)) ; } } if attr . description . is_none () { attr . description = Description :: parse_from_doc_attrs (attrs) ? ; } if attr . deprecated . is_none () { attr . deprecated = deprecation :: Directive :: parse_from_deprecated_attr (attrs) ? ; } Ok (attr) } }
};
}
