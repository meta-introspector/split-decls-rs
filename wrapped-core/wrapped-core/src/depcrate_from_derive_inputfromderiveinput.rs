// Generated macro for FromDeriveInput (trait)
macro_rules! Depcrate_from_derive_inputFromDeriveInput {
() => {
// Module: crate::from_derive_input
// Provides: {"FromDeriveInput"}
// Dependencies: {}
# [doc = " Creates an instance by parsing an entire proc-macro `derive` input,"] # [doc = " including the, identity, generics, and visibility of the type."] # [doc = ""] # [doc = " This trait should either be derived or manually implemented by a type"] # [doc = " in the proc macro crate which is directly using `darling`. It is unlikely"] # [doc = " that these implementations will be reusable across crates."] pub trait FromDeriveInput : Sized { # [doc = " Create an instance from `syn::DeriveInput`, or return an error."] fn from_derive_input (input : & DeriveInput) -> Result < Self > ; }
};
}
