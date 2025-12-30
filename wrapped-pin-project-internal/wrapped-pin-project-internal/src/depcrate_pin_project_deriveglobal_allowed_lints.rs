// Generated macro for global_allowed_lints (function)
macro_rules! Depcrate_pin_project_deriveglobal_allowed_lints {
() => {
// Module: crate::pin_project::derive
// Provides: {"global_allowed_lints"}
// Dependencies: {}
# [doc = " Returns attributes that should be applied to all generated code."] fn global_allowed_lints () -> TokenStream { quote ! { deprecated , explicit_outlives_requirements , single_use_lifetimes , unreachable_pub , unused_tuple_struct_fields , clippy :: unknown_clippy_lints , clippy :: absolute_paths , clippy :: min_ident_chars , clippy :: pattern_type_mismatch , clippy :: pub_with_shorthand , clippy :: redundant_pub_crate , clippy :: single_char_lifetime_names , clippy :: type_repetition_in_bounds , } }
};
}
