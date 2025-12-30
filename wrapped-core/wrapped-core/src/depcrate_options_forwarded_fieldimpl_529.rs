// Generated macro for impl_529 (impl)
macro_rules! Depcrate_options_forwarded_fieldimpl_529 {
() => {
// Module: crate::options::forwarded_field
// Provides: {"impl_529"}
// Dependencies: {}
impl ToTokens for Initializer < '_ > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let ident = & self . 0 . ident ; tokens . append_all (quote ! (# ident : # ident . expect ("Errors were already checked") ,)) ; } }
};
}
