use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// True if `c` is considered horizontal whitespace according to Rust language definition.
pub fn is_horizontal_whitespace(c: char) -> bool {
    matches!(c, '\u{0009}' | '\u{0020}')
}
