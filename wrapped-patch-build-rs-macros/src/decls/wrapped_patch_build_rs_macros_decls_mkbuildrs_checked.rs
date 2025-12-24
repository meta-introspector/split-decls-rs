use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl2(fn, name = "mkbuildrs", vis = "pub", hash = "1f3aae8e")]
pub fn mkbuildrs_checked(input: TokenStream) -> TokenStream {
    template_checker::mkbuildrs_checked_impl(input)
}
