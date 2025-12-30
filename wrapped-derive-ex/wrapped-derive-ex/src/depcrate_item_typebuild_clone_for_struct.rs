// Generated macro for build_clone_for_struct (function)
macro_rules! Depcrate_item_typebuild_clone_for_struct {
() => {
// Module: crate::item_type
// Provides: {"build_clone_for_struct"}
// Dependencies: {}
fn build_clone_for_struct (item : & ItemStruct , e : & DeriveEntry , fields : & [FieldEntry] ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Clone ; let (impl_g , type_g , _) = item . generics . split_for_impl () ; let this_ty_ident = & item . ident ; let this_ty : Type = parse_quote ! (# this_ty_ident # type_g) ; let trait_ = kind . to_path () ; let mut wcb = WhereClauseBuilder :: new (& item . generics) ; let use_bounds = e . push_bounds_to (& mut wcb) ; let mut ctor_args = Vec :: new () ; let mut clone_from_exprs = Vec :: new () ; for field in fields { let field_ty = & field . field . ty ; let lhs = & member (quote ! (self) , field) ; let rhs = & member (quote ! (source) , field) ; ctor_args . push (quote ! (<# field_ty as # trait_ >:: clone (&# lhs))) ; clone_from_exprs . push (quote ! (<# field_ty as # trait_ >:: clone_from (& mut # lhs , &# rhs))) ; field . push_bounds_to (use_bounds , kind , & mut wcb) ; } let ctor_args = build_ctor_args (& item . fields , & ctor_args) ; let wheres = wcb . build (| ty | quote ! (# ty : # trait_)) ; Ok (quote ! { # [automatically_derived] impl # impl_g # trait_ for # this_ty # wheres { fn clone (& self) -> Self { # this_ty_ident # ctor_args } fn clone_from (& mut self , source : & Self) { # (# clone_from_exprs ;) * } } }) }
};
}
