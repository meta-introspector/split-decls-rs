// Generated macro for impl_125 (impl)
macro_rules! Depcrate_codegen_fieldimpl_125 {
() => {
// Module: crate::codegen::field
// Provides: {"impl_125"}
// Dependencies: {}
impl ToTokens for CheckMissing < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { if ! self . 0 . multiple && self . 0 . default_expression . is_none () { let ident = self . 0 . ident ; let ty = self . 0 . ty ; let name_in_attr = & self . 0 . name_in_attr ; let from_none_call = quote_spanned ! (ty . span () => <# ty as :: darling :: FromMeta >:: from_none ()) ; tokens . append_all (quote ! { if !# ident . 0 { match # from_none_call { :: darling :: export :: Some (__type_fallback) => { # ident . 1 = :: darling :: export :: Some (__type_fallback) ; } :: darling :: export :: None => { __errors . push (:: darling :: Error :: missing_field (# name_in_attr)) } } } }) } } }
};
}
