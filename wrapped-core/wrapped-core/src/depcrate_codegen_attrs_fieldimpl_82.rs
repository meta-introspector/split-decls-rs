// Generated macro for impl_82 (impl)
macro_rules! Depcrate_codegen_attrs_fieldimpl_82 {
() => {
// Module: crate::codegen::attrs_field
// Provides: {"impl_82"}
// Dependencies: {}
impl ToTokens for Declaration < '_ > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let ident = & self . 0 . ident ; tokens . append_all (quote ! { let mut __fwd_attrs : :: darling :: export :: Vec <:: darling :: export :: syn :: Attribute > = vec ! [] ; let mut # ident : :: darling :: export :: Option < _ > = None ; }) ; } }
};
}
