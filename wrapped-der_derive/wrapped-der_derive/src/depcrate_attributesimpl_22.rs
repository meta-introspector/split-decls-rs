// Generated macro for impl_22 (impl)
macro_rules! Depcrate_attributesimpl_22 {
() => {
// Module: crate::attributes
// Provides: {"impl_22"}
// Dependencies: {}
impl TypeAttrs { # [doc = " Parse attributes from a struct field or enum variant."] pub fn parse (attrs : & [Attribute]) -> syn :: Result < Self > { let mut tag_mode = None ; let mut error = None ; attrs . iter () . try_for_each (| attr | { if ! attr . path () . is_ident (ATTR_NAME) { return Ok (()) ; } attr . parse_nested_meta (| meta | { if meta . path . is_ident ("tag_mode") { if tag_mode . is_some () { abort ! (attr , "duplicate ASN.1 `tag_mode` attribute") ; } tag_mode = Some (meta . value () ? . parse () ?) ; } else if meta . path . is_ident ("error") { if error . is_some () { abort ! (attr , "duplicate ASN.1 `error` attribute") ; } error = Some (ErrorType :: Custom (meta . value () ? . parse () ?)) ; } else { return Err (syn :: Error :: new_spanned (attr , "invalid `asn1` attribute (valid options are `tag_mode` and `error`)" ,)) ; } Ok (()) }) }) ? ; Ok (Self { tag_mode : tag_mode . unwrap_or_default () , error : error . unwrap_or_default () , }) } }
};
}
