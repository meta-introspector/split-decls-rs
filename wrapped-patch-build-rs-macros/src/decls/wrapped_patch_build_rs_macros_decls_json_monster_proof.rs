use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "monster_proof", vis = "pub", hash = "62b27247")]
pub fn json_monster_proof(input: TokenStream) -> TokenStream {
    lean4_json::json_monster_proof_impl(input)
}
