// Generated macro for impl_230 (impl)
macro_rules! Depcrate_graphql_interfaceimpl_230 {
() => {
// Module: crate::graphql_interface
// Provides: {"impl_230"}
// Dependencies: {}
impl Attr { # [doc = " Tries to merge two [`TraitAttr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { name : try_merge_opt ! (name : self , another) , description : try_merge_opt ! (description : self , another) , context : try_merge_opt ! (context : self , another) , scalar : try_merge_opt ! (scalar : self , another) , implemented_for : try_merge_hashset ! (implemented_for : self , another => span_joined) , implements : try_merge_hashset ! (implements : self , another => span_joined) , r#enum : try_merge_opt ! (r#enum : self , another) , asyncness : try_merge_opt ! (asyncness : self , another) , rename_fields : try_merge_opt ! (rename_fields : self , another) , is_internal : self . is_internal || another . is_internal , }) } # [doc = " Parses a [`TraitAttr`] from the provided multiple [`syn::Attribute`]s with"] # [doc = " the specified `names`, placed on a trait or struct definition."] fn from_attrs (names : impl AttrNames , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { let mut attr = filter_attrs (names , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) ? ; if attr . description . is_none () { attr . description = Description :: parse_from_doc_attrs (attrs) ? ; } Ok (attr) } }
};
}
