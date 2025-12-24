use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Removes all the color tags from the given string literal.
///
/// Accepts only one argument.
///
/// #### Example
///
/// ```
/// # use color_print_proc_macro::untagged;
/// let s: &str = untagged!("A <g>normal</> word");
/// assert_eq!(s, "A normal word");
/// ```
#[proc_macro]
pub fn untagged(input: TokenStream) -> TokenStream {
    crate::untagged::get_untagged(input)
        .unwrap_or_else(|err| err.to_token_stream())
        .into()
}
