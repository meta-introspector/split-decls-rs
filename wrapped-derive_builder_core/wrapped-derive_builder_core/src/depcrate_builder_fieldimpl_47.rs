// Generated macro for impl_47 (impl)
macro_rules! Depcrate_builder_fieldimpl_47 {
() => {
// Module: crate::builder_field
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > BuilderField < 'a > { # [doc = " Emits a struct field initializer that initializes the field to `Default::default`."] pub fn default_initializer_tokens (& self) -> TokenStream { let ident = self . field_ident ; let crate_root = self . crate_root ; quote ! { # ident : # crate_root :: export :: core :: default :: Default :: default () , } } }
};
}
