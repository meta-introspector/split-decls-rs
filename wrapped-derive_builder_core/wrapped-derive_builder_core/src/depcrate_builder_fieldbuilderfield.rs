// Generated macro for BuilderField (struct)
macro_rules! Depcrate_builder_fieldBuilderField {
() => {
// Module: crate::builder_field
// Provides: {"BuilderField"}
// Dependencies: {}
# [doc = " Field declaration for the builder struct, implementing `quote::ToTokens`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Will expand to something like the following (depending on settings):"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " # extern crate proc_macro2;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate quote;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate syn;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate derive_builder_core;"] # [doc = " # use derive_builder_core::{BuilderField, BuilderPattern};"] # [doc = " # fn main() {"] # [doc = " #    let attrs = vec![parse_quote!(#[some_attr])];"] # [doc = " #    let mut field = default_builder_field!();"] # [doc = " #    field.attrs = attrs.as_slice();"] # [doc = " #"] # [doc = " #    assert_eq!(quote!(#field).to_string(), quote!("] # [doc = " #[some_attr] pub foo: ::derive_builder::export::core::option::Option<String>,"] # [doc = " #    ).to_string());"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct BuilderField < 'a > { # [doc = " Path to the root of the derive_builder crate."] pub crate_root : & 'a syn :: Path , # [doc = " Name of the target field."] pub field_ident : & 'a syn :: Ident , # [doc = " Type of the builder field."] pub field_type : BuilderFieldType < 'a > , # [doc = " Visibility of this builder field, e.g. `syn::Visibility::Public`."] pub field_visibility : Cow < 'a , syn :: Visibility > , # [doc = " Attributes which will be attached to this builder field."] pub attrs : & 'a [syn :: Attribute] , }
};
}
