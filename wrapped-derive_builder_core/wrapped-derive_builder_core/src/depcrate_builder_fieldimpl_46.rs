// Generated macro for impl_46 (impl)
macro_rules! Depcrate_builder_fieldimpl_46 {
() => {
// Module: crate::builder_field
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a > ToTokens for BuilderField < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let ident = self . field_ident ; let vis = & self . field_visibility ; let ty = & self . field_type . with_crate_root (self . crate_root) ; let attrs = self . attrs ; tokens . append_all (quote ! (# (# attrs) * # vis # ident : # ty ,)) ; } }
};
}
