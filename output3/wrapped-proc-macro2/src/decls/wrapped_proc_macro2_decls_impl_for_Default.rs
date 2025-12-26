use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `TokenStream::default()` returns an empty stream,
/// i.e. this is equivalent with `TokenStream::new()`.
impl Default for TokenStream {
    fn default() -> Self {
        TokenStream::new()
    }
}
