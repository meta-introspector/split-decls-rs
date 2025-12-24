use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro_attribute]
pub fn test_internal(input: TokenStream, item: TokenStream) -> TokenStream {
    crate::executor::test(input, item)
}
