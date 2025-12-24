use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "lean4_patch", vis = "pub", hash = "b2bd9467")]
pub fn lean4_patch(input: TokenStream) -> TokenStream {
    lean4_json::lean4_patch_impl(input)
}
