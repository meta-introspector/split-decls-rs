// Generated macro for impl_187 (impl)
macro_rules! Depcrate_codegen_from_variant_implimpl_187 {
() => {
// Module: crate::codegen::from_variant_impl
// Provides: {"impl_187"}
// Dependencies: {}
impl ToTokens for FromVariantImpl < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let input = self . param_name () ; let extractor = self . extractor () ; let passed_ident = self . ident . as_ref () . map (| i | quote ! (# i : # input . ident . clone () ,)) ; let passed_discriminant = self . discriminant . as_ref () . map (| i | quote ! (# i : # input . discriminant . as_ref () . map (| (_ , expr) | expr . clone ()) ,)) ; let passed_attrs = self . forward_attrs . as_initializer () ; let passed_fields = self . fields . as_ref () . map (| i | quote ! (# i : :: darling :: ast :: Fields :: try_from (&# input . fields) ?,)) ; let inits = self . base . initializers () ; let post_transform = self . base . post_transform_call () ; let default = if self . from_ident { quote ! (let __default : Self = :: darling :: export :: From :: from (# input . ident . clone ()) ;) } else { self . base . fallback_decl () } ; let supports = self . supports . map (| i | { quote ! { __errors . handle (# i . check (&# input . fields)) ; } }) ; let error_declaration = self . base . declare_errors () ; let require_fields = self . base . require_fields () ; let error_check = self . base . check_errors () ; self . wrap (quote ! (fn from_variant (# input : &:: darling :: export :: syn :: Variant) -> :: darling :: Result < Self > { # error_declaration # extractor # supports # require_fields # error_check # default :: darling :: export :: Ok (Self { # passed_ident # passed_discriminant # passed_attrs # passed_fields # inits }) # post_transform }) , tokens ,) ; } }
};
}
