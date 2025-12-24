use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "context_optimize", vis = "pub", hash = "e731281e")]
pub fn context_optimize(input: TokenStream) -> TokenStream {
    context_knapsack::context_optimize_impl(input)
}
