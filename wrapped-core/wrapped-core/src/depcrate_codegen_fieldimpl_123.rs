// Generated macro for impl_123 (impl)
macro_rules! Depcrate_codegen_fieldimpl_123 {
() => {
// Module: crate::codegen::field
// Provides: {"impl_123"}
// Dependencies: {}
impl ToTokens for Initializer < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let field = self . 0 ; let ident = field . ident ; tokens . append_all (if field . multiple { if let Some (ref expr) = field . default_expression { quote_spanned ! (expr . span () => # ident : if !# ident . is_empty () { # ident } else { # expr }) } else { quote ! (# ident : # ident) } } else if let Some (ref expr) = field . default_expression { quote_spanned ! (expr . span () => # ident : if let Some (__val) = # ident . 1 { __val } else { # expr }) } else { quote ! (# ident : # ident . 1 . expect ("Uninitialized fields without defaults were already checked")) }) ; } }
};
}
