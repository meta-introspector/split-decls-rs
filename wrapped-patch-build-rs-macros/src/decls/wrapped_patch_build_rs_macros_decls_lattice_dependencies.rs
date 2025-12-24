use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "lattice_dependencies", vis = "pub", hash = "6283aa32")]
pub fn lattice_dependencies(input: TokenStream) -> TokenStream {
    macro_lattice::lattice_dependencies_impl(input)
}
