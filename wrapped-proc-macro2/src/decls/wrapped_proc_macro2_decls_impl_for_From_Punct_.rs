use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Punct> for TokenTree {
    fn from(g: Punct) -> Self {
        TokenTree::Punct(g)
    }
}
