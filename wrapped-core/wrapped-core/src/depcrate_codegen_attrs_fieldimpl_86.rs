// Generated macro for impl_86 (impl)
macro_rules! Depcrate_codegen_attrs_fieldimpl_86 {
() => {
// Module: crate::codegen::attrs_field
// Provides: {"impl_86"}
// Dependencies: {}
impl ToTokens for MatchArms < '_ > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { if ! self . 0 . will_forward_any () { tokens . append_all (quote ! (_ => continue)) ; return ; } let push_command = quote ! (__fwd_attrs . push (__attr . clone ())) ; tokens . append_all (match self . 0 . filter . expect ("Can only forward attributes if filter is defined") { ForwardAttrsFilter :: All => quote ! (_ => # push_command) , ForwardAttrsFilter :: Only (idents) => { let names = idents . to_strings () ; quote ! { # (# names) |* => # push_command , _ => continue , } } } ,) ; } }
};
}
