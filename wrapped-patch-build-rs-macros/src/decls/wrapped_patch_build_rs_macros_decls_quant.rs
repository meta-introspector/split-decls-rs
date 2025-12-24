use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "quant", vis = "pub", hash = "47276e9c")]
pub fn quant(input: TokenStream) -> TokenStream {
    quant_trading::quant_impl(input)
}
