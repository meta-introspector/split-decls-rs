// Generated macro for Bake (trait)
macro_rules! DepcrateBake {
() => {
// Module: crate
// Provides: {"Bake"}
// Dependencies: {}
# [doc = " The `Bake` trait allows a piece of data to write itself into a Rust expression."] # [doc = ""] # [doc = " This can be used to generate files with hardcoded data."] pub trait Bake { # [doc = " Returns a [`TokenStream`] that would evaluate to `self`."] # [doc = ""] # [doc = " Crates that are required for the evaluation of the [`TokenStream`] will be"] # [doc = " added to `ctx`."] fn bake (& self , ctx : & CrateEnv) -> TokenStream ; }
};
}
