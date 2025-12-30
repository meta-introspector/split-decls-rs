// Generated macro for impl_132 (impl)
macro_rules! Depcrate_codegen_from_attributes_implimpl_132 {
() => {
// Module: crate::codegen::from_attributes_impl
// Provides: {"impl_132"}
// Dependencies: {}
impl ToTokens for FromAttributesImpl < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let ty_ident = self . base . ident ; let input = self . param_name () ; let post_transform = self . base . post_transform_call () ; if let Data :: Struct (ref data) = self . base . data { if data . is_newtype () { self . wrap (quote ! { fn from_attributes (# input : & [:: darling :: export :: syn :: Attribute]) -> :: darling :: Result < Self > { :: darling :: export :: Ok (# ty_ident (:: darling :: FromAttributes :: from_attributes (# input) ?)) # post_transform } } , tokens ,) ; return ; } } let passed_attrs = self . forward_attrs . as_initializer () ; let inits = self . base . initializers () ; let default = self . base . fallback_decl () ; let grab_attrs = self . extractor () ; let declare_errors = self . base . declare_errors () ; let require_fields = self . base . require_fields () ; let check_errors = self . base . check_errors () ; self . wrap (quote ! { fn from_attributes (# input : & [:: darling :: export :: syn :: Attribute]) -> :: darling :: Result < Self > { # declare_errors # grab_attrs # require_fields # check_errors # default :: darling :: export :: Ok (# ty_ident { # passed_attrs # inits }) # post_transform } } , tokens ,) ; } }
};
}
