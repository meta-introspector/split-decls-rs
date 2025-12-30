// Generated macro for build_debug_for_struct (function)
macro_rules! Depcrate_item_typebuild_debug_for_struct {
() => {
// Module: crate::item_type
// Provides: {"build_debug_for_struct"}
// Dependencies: {}
fn build_debug_for_struct (item : & ItemStruct , e : & DeriveEntry , hattrs : & HelperAttributes , fields : & [FieldEntry] ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Debug ; let (impl_g , type_g , _) = item . generics . split_for_impl () ; let this_ty_ident = & item . ident ; let this_ty : Type = parse_quote ! (# this_ty_ident # type_g) ; let trait_ = kind . to_path () ; let mut wcb = WhereClauseBuilder :: new (& item . generics) ; let use_bounds = e . push_bounds_to_with (hattrs , kind , & mut wcb) ; let to_expr = | field : & FieldEntry | { let member = field . member () ; quote_spanned ! (field . span () => & self .# member) } ; let expr = build_debug_expr (this_ty_ident , & item . fields , fields , use_bounds , to_expr , & mut wcb ,) ? ; let wheres = wcb . build (| ty | quote ! (# ty : # trait_)) ; Ok (quote ! { # [automatically_derived] impl # impl_g # trait_ for # this_ty # wheres { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { # expr } } }) }
};
}
