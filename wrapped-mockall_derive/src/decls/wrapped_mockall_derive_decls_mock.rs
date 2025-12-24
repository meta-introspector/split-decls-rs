use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
pub fn mock(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    do_mock(input.into()).into()
}
