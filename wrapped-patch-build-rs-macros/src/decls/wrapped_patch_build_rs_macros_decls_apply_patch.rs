use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "apply_patch", vis = "pub", hash = "1370bee3")]
pub fn apply_patch(input: TokenStream) -> TokenStream {
    dao_governance::apply_patch_impl(input)
}
