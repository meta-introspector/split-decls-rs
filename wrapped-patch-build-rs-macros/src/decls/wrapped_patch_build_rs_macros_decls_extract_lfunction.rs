use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "extract_lfunction", vis = "pub", hash = "0bd53dc8")]
pub fn extract_lfunction(input: TokenStream) -> TokenStream {
    sat_lfunction::extract_lfunction_impl(input)
}
