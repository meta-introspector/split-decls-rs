// Generated macro for impl_227 (impl)
macro_rules! Depcrate_codegen_variantimpl_227 {
() => {
// Module: crate::codegen::variant
// Provides: {"impl_227"}
// Dependencies: {}
impl ToTokens for UnitMatchArm < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let val : & Variant < '_ > = self . 0 ; if val . skip { return ; } let name_in_attr = & val . name_in_attr ; let unsupported_format_error = | | { quote ! (:: darling :: export :: Err (:: darling :: Error :: unsupported_format ("literal"))) } ; if val . data . is_unit () { let variant_ident = val . variant_ident ; let ty_ident = val . ty_ident ; tokens . append_all (quote ! (# name_in_attr => :: darling :: export :: Ok (# ty_ident ::# variant_ident) ,)) ; } else if val . data . is_newtype () { let field = val . data . fields . first () . expect ("Newtype should have exactly one field") ; let field_ty = field . ty ; let ty_ident = val . ty_ident ; let variant_ident = val . variant_ident ; let unsupported_format = unsupported_format_error () ; tokens . append_all (quote ! { # name_in_attr => { match <# field_ty as :: darling :: FromMeta >:: from_none () { :: darling :: export :: Some (__value) => :: darling :: export :: Ok (# ty_ident ::# variant_ident (__value)) , :: darling :: export :: None => # unsupported_format , } } }) } else { let unsupported_format = unsupported_format_error () ; tokens . append_all (quote ! (# name_in_attr => # unsupported_format ,)) ; } } }
};
}
