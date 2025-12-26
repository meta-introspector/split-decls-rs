use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The `try_join!` macro.
#[proc_macro]
pub fn try_join_internal(input: TokenStream) -> TokenStream {
    crate::join::try_join(input)
}
