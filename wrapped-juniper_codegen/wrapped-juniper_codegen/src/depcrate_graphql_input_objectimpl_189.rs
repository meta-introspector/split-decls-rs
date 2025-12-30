// Generated macro for impl_189 (impl)
macro_rules! Depcrate_graphql_input_objectimpl_189 {
() => {
// Module: crate::graphql_input_object
// Provides: {"impl_189"}
// Dependencies: {}
impl ContainerAttr { # [doc = " Tries to merge two [`ContainerAttr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { name : try_merge_opt ! (name : self , another) , description : try_merge_opt ! (description : self , another) , context : try_merge_opt ! (context : self , another) , scalar : try_merge_opt ! (scalar : self , another) , rename_fields : try_merge_opt ! (rename_fields : self , another) , is_internal : self . is_internal || another . is_internal , }) } # [doc = " Parses [`ContainerAttr`] from the given multiple `name`d"] # [doc = " [`syn::Attribute`]s placed on a struct or impl block definition."] pub (crate) fn from_attrs (name : & str , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { let mut attr = filter_attrs (name , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) ? ; if attr . description . is_none () { attr . description = Description :: parse_from_doc_attrs (attrs) ? ; } Ok (attr) } }
};
}
