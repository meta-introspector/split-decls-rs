use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error returned from `TokenStream::from_str`.
pub struct LexError {
    inner: imp::LexError,
    _marker: ProcMacroAutoTraits,
}
