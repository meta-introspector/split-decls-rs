use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Colorizes a string literal, without formatting the `format!`-like placeholders.
///
/// * Accepts only one argument;
/// * Will panic if feature `terminfo` is activated.
#[cfg(feature = "terminfo")]
#[proc_macro]
pub fn cstr(_: TokenStream) -> TokenStream {
    panic!("Macro cstr!() cannot be used with terminfo feature")
}
