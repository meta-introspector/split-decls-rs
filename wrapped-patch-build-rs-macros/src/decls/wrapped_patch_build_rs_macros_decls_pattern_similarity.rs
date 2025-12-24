use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "pattern_similarity", vis = "pub", hash = "cd91f864")]
pub fn pattern_similarity(input: TokenStream) -> TokenStream {
    repo_analysis::pattern_similarity_impl(input)
}
