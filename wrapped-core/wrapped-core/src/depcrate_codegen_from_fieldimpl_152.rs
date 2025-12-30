// Generated macro for impl_152 (impl)
macro_rules! Depcrate_codegen_from_fieldimpl_152 {
() => {
// Module: crate::codegen::from_field
// Provides: {"impl_152"}
// Dependencies: {}
impl ToTokens for FromFieldImpl < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let input = self . param_name () ; let error_declaration = self . base . declare_errors () ; let require_fields = self . base . require_fields () ; let error_check = self . base . check_errors () ; let initializers = self . base . initializers () ; let default = if self . from_ident { quote ! (let __default : Self = :: darling :: export :: From :: from (# input . ident . clone ()) ;) } else { self . base . fallback_decl () } ; let passed_ident = self . ident . as_ref () . map (| i | quote ! (# i : # input . ident . clone () ,)) ; let passed_vis = self . vis . as_ref () . map (| i | quote ! (# i : # input . vis . clone () ,)) ; let passed_ty = self . ty . as_ref () . map (| i | quote ! (# i : # input . ty . clone () ,)) ; let passed_attrs = self . forward_attrs . as_initializer () ; let grab_attrs = self . extractor () ; let post_transform = self . base . post_transform_call () ; self . wrap (quote ! { fn from_field (# input : &:: darling :: export :: syn :: Field) -> :: darling :: Result < Self > { # error_declaration # grab_attrs # require_fields # error_check # default :: darling :: export :: Ok (Self { # passed_ident # passed_ty # passed_vis # passed_attrs # initializers }) # post_transform } } , tokens ,) ; } }
};
}
