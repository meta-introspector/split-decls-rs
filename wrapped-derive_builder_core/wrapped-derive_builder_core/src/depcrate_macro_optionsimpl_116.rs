// Generated macro for impl_116 (impl)
macro_rules! Depcrate_macro_optionsimpl_116 {
() => {
// Module: crate::macro_options
// Provides: {"impl_116"}
// Dependencies: {}
impl Field { # [doc = " Resolve and check (post-parsing) options which come from multiple darling options"] # [doc = ""] # [doc = "  * Check that we don't have a custom field type or builder *and* a default value"] fn resolve (self) -> darling :: Result < Self > { let mut errors = darling :: Error :: accumulator () ; if let Field { default : Some (field_default) , .. } = & self { if self . field . build . is_some () { errors . push (darling :: Error :: custom (r#"#[builder(default)] and #[builder(field(build="..."))] cannot be used together"# ,) . with_span (& field_default . span ()) ,) ; } if self . field . builder_type . is_some () { errors . push (darling :: Error :: custom (r#"#[builder(default)] and #[builder(field(ty="..."))] cannot be used together"# ,) . with_span (& field_default . span ())) } } ; errors . finish_with (self) } }
};
}
