use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn process_decl2_attribute_logic(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let args = parse_decl_args!(attr);
    dispatch_wrap_logic!(item, args)
}
