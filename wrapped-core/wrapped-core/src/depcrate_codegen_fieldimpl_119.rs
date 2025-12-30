// Generated macro for impl_119 (impl)
macro_rules! Depcrate_codegen_fieldimpl_119 {
() => {
// Module: crate::codegen::field
// Provides: {"impl_119"}
// Dependencies: {}
impl ToTokens for FlattenInitializer < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let Self { field , parent_field_names , } = self ; let ident = field . ident ; let add_parent_fields = if parent_field_names . is_empty () { None } else { Some (quote ! { . map_err (| e | e . add_sibling_alts_for_unknown_field (& [# (# parent_field_names) ,*])) }) } ; tokens . append_all (quote ! { # ident = (true , __errors . handle (:: darling :: FromMeta :: from_list (& __flatten) # add_parent_fields)) ; }) ; } }
};
}
