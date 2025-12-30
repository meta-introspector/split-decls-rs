// Generated macro for impl_300 (impl)
macro_rules! Depcrate_graphql_scalarimpl_300 {
() => {
// Module: crate::graphql_scalar
// Provides: {"impl_300"}
// Dependencies: {}
impl Attr { # [doc = " Tries to merge two [`Attr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { name : try_merge_opt ! (name : self , another) , description : try_merge_opt ! (description : self , another) , specified_by_url : try_merge_opt ! (specified_by_url : self , another) , scalar : try_merge_opt ! (scalar : self , another) , to_output : try_merge_opt ! (to_output : self , another) , from_input : try_merge_opt ! (from_input : self , another) , parse_token : try_merge_opt ! (parse_token : self , another) , with : try_merge_opt ! (with : self , another) , where_clause : try_merge_opt ! (where_clause : self , another) , transparent : self . transparent || another . transparent , }) } # [doc = " Parses an [`Attr`] from the provided multiple [`syn::Attribute`]s with"] # [doc = " the specified `names`, placed on a type definition."] fn from_attrs (names : impl AttrNames , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { let mut attr = filter_attrs (names , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) ? ; if attr . description . is_none () { attr . description = Description :: parse_from_doc_attrs (attrs) ? ; } Ok (attr) } }
};
}
