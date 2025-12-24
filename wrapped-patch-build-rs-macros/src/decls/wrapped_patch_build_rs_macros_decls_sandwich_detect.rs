use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "sandwich_detect", vis = "pub", hash = "639c2145")]
pub fn sandwich_detect(input: TokenStream) -> TokenStream {
    mev_protection::sandwich_detect_impl(input)
}
