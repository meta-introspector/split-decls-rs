use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "formal_verification", vis = "pub", hash = "7d168d21")]
pub fn formal_verification(input: TokenStream) -> TokenStream {
    lean4_proof::formal_verification_impl(input)
}
