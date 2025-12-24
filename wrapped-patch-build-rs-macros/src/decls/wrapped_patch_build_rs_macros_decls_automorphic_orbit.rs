use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "automorphic_orbit", vis = "pub", hash = "06a5946e")]
pub fn automorphic_orbit(input: TokenStream) -> TokenStream {
    quine_relay::automorphic_orbit_impl(input)
}
