// Generated macro for build_copy_for_struct (function)
macro_rules! Depcrate_item_typebuild_copy_for_struct {
() => {
// Module: crate::item_type
// Provides: {"build_copy_for_struct"}
// Dependencies: {}
fn build_copy_for_struct (item : & ItemStruct , e : & DeriveEntry , fields : & [FieldEntry] ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Copy ; let (impl_g , type_g , _) = item . generics . split_for_impl () ; let this_ty_ident = & item . ident ; let this_ty : Type = parse_quote ! (# this_ty_ident # type_g) ; let trait_ = kind . to_path () ; let mut wcb = WhereClauseBuilder :: new (& item . generics) ; let use_bounds = e . push_bounds_to (& mut wcb) ; for field in fields { field . push_bounds_to (use_bounds , kind , & mut wcb) ; } let wheres = wcb . build (| ty | quote ! (# ty : # trait_)) ; Ok (quote ! { # [automatically_derived] impl # impl_g # trait_ for # this_ty # wheres { } }) }
};
}
