use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The `select_biased!` macro.
#[proc_macro]
pub fn select_biased_internal(input: TokenStream) -> TokenStream {
    crate::select::select_biased(input)
}
