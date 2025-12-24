use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "code_split", vis = "pub", hash = "489e398b")]
pub fn code_split(input: TokenStream) -> TokenStream {
    graph_partition::code_split_impl(input)
}
