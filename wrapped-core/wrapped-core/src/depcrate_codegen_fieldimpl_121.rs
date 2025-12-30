// Generated macro for impl_121 (impl)
macro_rules! Depcrate_codegen_fieldimpl_121 {
() => {
// Module: crate::codegen::field
// Provides: {"impl_121"}
// Dependencies: {}
impl ToTokens for MatchArm < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let field = self . 0 ; if field . skip || field . flatten { return ; } let name_str = & field . name_in_attr ; let ident = field . ident ; let with_callable = & field . with_callable ; let post_transform = field . post_transform . as_ref () ; let location = if field . multiple { quote ! (& format ! ("{}[{}]" , # name_str , __len)) } else { quote ! (# name_str) } ; let extractor = quote_spanned ! (with_callable . span () => :: darling :: export :: identity ::< fn (&:: darling :: export :: syn :: Meta) -> :: darling :: Result < _ >> (# with_callable) (__inner) # post_transform . map_err (| e | e . with_span (& __inner) . at (# location))) ; tokens . append_all (if field . multiple { quote ! (# name_str => { let __len = # ident . len () ; if let :: darling :: export :: Some (__val) = __errors . handle (# extractor) { # ident . push (__val) } }) } else { quote ! (# name_str => { if !# ident . 0 { # ident = (true , __errors . handle (# extractor)) ; } else { __errors . push (:: darling :: Error :: duplicate_field (# name_str) . with_span (& __inner)) ; } }) }) ; } }
};
}
