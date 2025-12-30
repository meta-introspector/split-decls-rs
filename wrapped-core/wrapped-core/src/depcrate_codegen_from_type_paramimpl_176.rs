// Generated macro for impl_176 (impl)
macro_rules! Depcrate_codegen_from_type_paramimpl_176 {
() => {
// Module: crate::codegen::from_type_param
// Provides: {"impl_176"}
// Dependencies: {}
impl ToTokens for FromTypeParamImpl < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let input = self . param_name () ; let error_declaration = self . base . declare_errors () ; let grab_attrs = self . extractor () ; let require_fields = self . base . require_fields () ; let error_check = self . base . check_errors () ; let default = if self . from_ident { quote ! (let __default : Self = :: darling :: export :: From :: from (# input . ident . clone ()) ;) } else { self . base . fallback_decl () } ; let passed_ident = self . ident . as_ref () . map (| i | quote ! (# i : # input . ident . clone () ,)) ; let passed_attrs = self . forward_attrs . as_initializer () ; let passed_bounds = self . bounds . as_ref () . map (| i | quote ! (# i : # input . bounds . clone () . into_iter () . collect ::< Vec < _ >> () ,)) ; let passed_default = self . default . as_ref () . map (| i | quote ! (# i : # input . default . clone () ,)) ; let initializers = self . base . initializers () ; let post_transform = self . base . post_transform_call () ; self . wrap (quote ! { fn from_type_param (# input : &:: darling :: export :: syn :: TypeParam) -> :: darling :: Result < Self > { # error_declaration # grab_attrs # require_fields # error_check # default :: darling :: export :: Ok (Self { # passed_ident # passed_bounds # passed_default # passed_attrs # initializers }) # post_transform } } , tokens ,) ; } }
};
}
