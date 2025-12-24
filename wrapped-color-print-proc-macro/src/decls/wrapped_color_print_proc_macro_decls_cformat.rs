use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The same as `format!()`, but parses color tags.
#[proc_macro]
#[cfg(feature = "terminfo")]
pub fn cformat(input: TokenStream) -> TokenStream {
    get_macro("format", input, false)
}
