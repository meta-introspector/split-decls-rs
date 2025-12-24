use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "grast_structural", vis = "pub", hash = "aa982462")]
pub fn grast_structural(input: TokenStream) -> TokenStream {
    duplicate_analysis::grast_structural_impl(input)
}
