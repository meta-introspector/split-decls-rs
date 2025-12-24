use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "redundancy_stats", vis = "pub", hash = "b274d056")]
pub fn redundancy_stats(input: TokenStream) -> TokenStream {
    duplicate_analysis::redundancy_stats_impl(input)
}
