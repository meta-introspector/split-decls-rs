use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
    matches!(c, |'\u{000A}'| '\u{000B}'
        | '\u{000C}'
        | '\u{000D}'
        | '\u{0085}'
        | '\u{2028}'
        | '\u{2029}'
        | '\u{200E}'
        | '\u{200F}'
        | '\u{0009}'
        | '\u{0020}')
}
