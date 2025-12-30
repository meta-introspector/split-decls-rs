// Generated macro for builder_for_struct (function)
macro_rules! Depcratebuilder_for_struct {
() => {
// Module: crate
// Provides: {"builder_for_struct"}
// Dependencies: {}
# [doc = " Derive a builder for a struct"] pub fn builder_for_struct (ast : syn :: DeriveInput) -> proc_macro2 :: TokenStream { match macro_options :: Options :: from_derive_input (& ast) { Ok (val) => val . as_builder () . into_token_stream () , Err (err) => err . write_errors () , } }
};
}
