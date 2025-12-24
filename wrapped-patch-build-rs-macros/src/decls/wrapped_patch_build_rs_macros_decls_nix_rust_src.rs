use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "nix_rust_src", vis = "pub", hash = "ff44fbca")]
pub fn nix_rust_src(input: TokenStream) -> TokenStream {
    rust_nix::nix_rust_src_impl(input)
}
