use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl iter::FromIterator<char> for SmolStr {
    fn from_iter<I: iter::IntoIterator<Item = char>>(iter: I) -> SmolStr {
        from_char_iter(iter.into_iter())
    }
}
