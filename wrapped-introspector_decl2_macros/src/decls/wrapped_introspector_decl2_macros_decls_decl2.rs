use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro_attribute]
pub fn decl2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_2 = proc_macro2::TokenStream::from(attr);
    let item_2 = proc_macro2::TokenStream::from(item);
    introspector_decl_core::process_decl2_attribute_logic(attr_2, item_2).into()
}
