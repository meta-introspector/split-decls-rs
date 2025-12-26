use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
    symbols::symbols(input.into()).into()
}
