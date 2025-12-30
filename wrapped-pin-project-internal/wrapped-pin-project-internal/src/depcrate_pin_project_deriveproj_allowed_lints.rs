// Generated macro for proj_allowed_lints (function)
macro_rules! Depcrate_pin_project_deriveproj_allowed_lints {
() => {
// Module: crate::pin_project::derive
// Provides: {"proj_allowed_lints"}
// Dependencies: {}
# [doc = " Returns attributes used on projected types."] fn proj_allowed_lints (cx : & Context < '_ >) -> (TokenStream , TokenStream , TokenStream) { let global_allowed_lints = global_allowed_lints () ; let proj_mut_allowed_lints = if cx . project { Some (& global_allowed_lints) } else { None } ; let proj_mut = quote ! { # [allow (dead_code , # proj_mut_allowed_lints clippy :: missing_docs_in_private_items , clippy :: mut_mut)] } ; let proj_ref_allowed_lints = if cx . project_ref { Some (& global_allowed_lints) } else { None } ; let proj_ref = quote ! { # [allow (dead_code , # proj_ref_allowed_lints clippy :: missing_docs_in_private_items , clippy :: ref_option_ref)] } ; let proj_own_allowed_lints = if cx . project_replace . ident () . is_some () { Some (& global_allowed_lints) } else { None } ; let variant_size_differences = if cx . kind == Enum { Some (quote ! { variant_size_differences , clippy :: large_enum_variant , }) } else { None } ; let proj_own = quote ! { # [allow (dead_code , # proj_own_allowed_lints # variant_size_differences clippy :: missing_docs_in_private_items)] } ; (proj_mut , proj_ref , proj_own) }
};
}
