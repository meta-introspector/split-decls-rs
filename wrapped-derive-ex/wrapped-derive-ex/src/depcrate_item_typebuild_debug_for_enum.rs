// Generated macro for build_debug_for_enum (function)
macro_rules! Depcrate_item_typebuild_debug_for_enum {
() => {
// Module: crate::item_type
// Provides: {"build_debug_for_enum"}
// Dependencies: {}
fn build_debug_for_enum (item : & ItemEnum , e : & DeriveEntry , hattrs : & HelperAttributes , variants : & [VariantEntry] ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Debug ; let (impl_g , type_g , _) = item . generics . split_for_impl () ; let this_ty_ident = & item . ident ; let this_ty : Type = parse_quote ! (# this_ty_ident # type_g) ; let trait_ = kind . to_path () ; let mut wcb = WhereClauseBuilder :: new (& item . generics) ; let use_bounds = e . push_bounds_to_with (hattrs , kind , & mut wcb) ; let mut arms = Vec :: new () ; for variant in variants { let variant_ident = & variant . variant . ident ; let use_bounds = variant . hattrs . push_bounds_to (use_bounds , kind , & mut wcb) ; let to_expr = | field : & FieldEntry | { let var = field . make_ident ("") ; quote_spanned ! (field . span () => # var) } ; let expr = build_debug_expr (variant_ident , & variant . variant . fields , & variant . fields , use_bounds , to_expr , & mut wcb ,) ? ; let pat = variant . make_pat ("") ; arms . push (quote ! (# pat => # expr)) ; } let wheres = wcb . build (| ty | quote ! (# ty : # trait_)) ; Ok (quote ! { # [automatically_derived] impl # impl_g # trait_ for # this_ty # wheres { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { match self { # (# arms ,) * } } } }) }
};
}
