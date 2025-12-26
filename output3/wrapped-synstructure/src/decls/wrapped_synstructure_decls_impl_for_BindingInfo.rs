use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> BindingInfo<'a> {
    /// Returns a reference to the underlying `syn` AST node which this
    /// `BindingInfo` references
    pub fn ast(&self) -> &'a Field {
        self.field
    }
    /// Generates the pattern fragment for this field binding.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     enum A {
    ///         B{ a: i32, b: i32 },
    ///         C(u32),
    ///     }
    /// };
    /// let s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].bindings()[0].pat().to_string(),
    ///     quote! {
    ///         ref __binding_0
    ///     }.to_string()
    /// );
    /// ```
    pub fn pat(&self) -> TokenStream {
        let BindingInfo { binding, style, .. } = self;
        quote!(# style # binding)
    }
    /// Returns a list of the type parameters which are referenced in this
    /// field's type.
    ///
    /// # Caveat
    ///
    /// If the field contains any macros in type position, all parameters will
    /// be considered bound. This is because we cannot determine which type
    /// parameters are bound by type macros.
    ///
    /// # Example
    /// ```
    /// # use synstructure::*;
    /// let di: syn::DeriveInput = syn::parse_quote! {
    ///     struct A<T, U> {
    ///         a: Option<T>,
    ///         b: U,
    ///     }
    /// };
    /// let mut s = Structure::new(&di);
    ///
    /// assert_eq!(
    ///     s.variants()[0].bindings()[0].referenced_ty_params(),
    ///     &[&quote::format_ident!("T")]
    /// );
    /// ```
    pub fn referenced_ty_params(&self) -> Vec<&'a Ident> {
        fetch_generics(&self.seen_generics, self.generics)
    }
}
