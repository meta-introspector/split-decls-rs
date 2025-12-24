use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "backpack_fill", vis = "pub", hash = "d49e4961")]
pub fn backpack_fill(input: TokenStream) -> TokenStream {
    context_knapsack::backpack_fill_impl(input)
}
