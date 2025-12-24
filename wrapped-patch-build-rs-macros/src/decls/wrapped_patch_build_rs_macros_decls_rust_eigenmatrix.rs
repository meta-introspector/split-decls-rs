use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "rust_eigenmatrix", vis = "pub", hash = "53721425")]
pub fn rust_eigenmatrix(input: TokenStream) -> TokenStream {
    rust_eigenmatrix::rust_eigenmatrix_impl(input)
}
