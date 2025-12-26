use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Type of a struct field or tuple variant field in the syntax tree.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Type {
    /// Syntax tree type defined by Syn.
    ///
    /// This name will match the ident of some `Node`.
    Syn(String),
    /// Type defined by the Rust language or standard library.
    ///
    /// All such types used by Syn are accessible in the Rust prelude and can be
    /// used without a qualifying path in most Rust code.
    Std(String),
    /// Type defined by proc-macro2.
    ///
    /// The type is accessible in the proc-macro2 public API as
    /// `proc_macro2::#name`.
    #[cfg_attr(feature = "serde", serde(rename = "proc_macro2"))]
    Ext(String),
    /// Keyword or punctuation token type defined by Syn.
    ///
    /// This name will match one of the keys in the `tokens` map.
    Token(String),
    /// Grouping token defined by Syn.
    ///
    /// The type is accessible in the Syn public API as `syn::token::#name`.
    Group(String),
    /// Punctuated list.
    ///
    /// This refers to `syn::punctuated::Punctuated<T, P>` with the specified
    /// element type and punctuation.
    Punctuated(Punctuated),
    /// `std::option::Option`
    Option(Box<Type>),
    /// `std::boxed::Box`
    Box(Box<Type>),
    /// `std::vec::Vec`
    Vec(Box<Type>),
    /// Rust tuple with two or more fields.
    Tuple(Vec<Type>),
}
