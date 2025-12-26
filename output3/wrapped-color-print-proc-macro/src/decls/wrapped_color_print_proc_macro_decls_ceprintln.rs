use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The same as `eprintln!()`, but parses color tags.
#[proc_macro]
#[cfg(feature = "terminfo")]
pub fn ceprintln(input: TokenStream) -> TokenStream {
    get_macro("eprintln", input, false)
}
