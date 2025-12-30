// Generated macro for build_default_for_struct (function)
macro_rules! Depcrate_item_typebuild_default_for_struct {
() => {
// Module: crate::item_type
// Provides: {"build_default_for_struct"}
// Dependencies: {}
fn build_default_for_struct (item : & ItemStruct , e : & DeriveEntry , hattrs : & HelperAttributes , fields : & [FieldEntry] ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Default ; let (impl_g , type_g , _) = item . generics . split_for_impl () ; let this_ty_ident = & item . ident ; let this_ty : Type = parse_quote ! (# this_ty_ident # type_g) ; let trait_ = kind . to_path () ; let mut wcb = WhereClauseBuilder :: new (& item . generics) ; let use_bounds = e . push_bounds_to_with (hattrs , kind , & mut wcb) ; let value = hattrs . default . as_ref () . and_then (| a | a . value (& parse_quote ! (Self))) ; let value = if let Some (value) = value { value } else { let ctor_args = build_default_ctor_args (& item . fields , fields , use_bounds , & mut wcb) ? ; quote ! (# this_ty_ident # ctor_args) } ; let wheres = wcb . build (| ty | quote ! (# ty : # trait_)) ; Ok (quote ! { # [automatically_derived] impl # impl_g # trait_ for # this_ty # wheres { fn default () -> Self { # value } } }) }
};
}
