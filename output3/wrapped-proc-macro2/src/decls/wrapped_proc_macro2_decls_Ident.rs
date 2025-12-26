use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A word of Rust code, which may be a keyword or legal variable name.
///
/// An identifier consists of at least one Unicode code point, the first of
/// which has the XID_Start property and the rest of which have the XID_Continue
/// property.
///
/// - The empty string is not an identifier. Use `Option<Ident>`.
/// - A lifetime is not an identifier. Use `syn::Lifetime` instead.
///
/// An identifier constructed with `Ident::new` is permitted to be a Rust
/// keyword, though parsing one through its [`Parse`] implementation rejects
/// Rust keywords. Use `input.call(Ident::parse_any)` when parsing to match the
/// behaviour of `Ident::new`.
///
/// [`Parse`]: https://docs.rs/syn/2.0/syn/parse/trait.Parse.html
///
/// # Examples
///
/// A new ident can be created from a string using the `Ident::new` function.
/// A span must be provided explicitly which governs the name resolution
/// behavior of the resulting identifier.
///
/// ```
/// use proc_macro2::{Ident, Span};
///
/// fn main() {
///     let call_ident = Ident::new("calligraphy", Span::call_site());
///
///     println!("{}", call_ident);
/// }
/// ```
///
/// An ident can be interpolated into a token stream using the `quote!` macro.
///
/// ```
/// use proc_macro2::{Ident, Span};
/// use quote::quote;
///
/// fn main() {
///     let ident = Ident::new("demo", Span::call_site());
///
///     // Create a variable binding whose name is this ident.
///     let expanded = quote! { let #ident = 10; };
///
///     // Create a variable binding with a slightly different name.
///     let temp_ident = Ident::new(&format!("new_{}", ident), Span::call_site());
///     let expanded = quote! { let #temp_ident = 10; };
/// }
/// ```
///
/// A string representation of the ident is available through the `to_string()`
/// method.
///
/// ```
/// # use proc_macro2::{Ident, Span};
/// #
/// # let ident = Ident::new("another_identifier", Span::call_site());
/// #
/// // Examine the ident as a string.
/// let ident_string = ident.to_string();
/// if ident_string.len() > 60 {
///     println!("Very long identifier: {}", ident_string)
/// }
/// ```
#[derive(Clone)]
pub struct Ident {
    inner: imp::Ident,
    _marker: ProcMacroAutoTraits,
}
