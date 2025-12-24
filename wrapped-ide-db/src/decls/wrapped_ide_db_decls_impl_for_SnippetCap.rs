use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SnippetCap {
    pub const fn new(allow_snippets: bool) -> Option<SnippetCap> {
        if allow_snippets { Some(SnippetCap { _private: () }) } else { None }
    }
}
