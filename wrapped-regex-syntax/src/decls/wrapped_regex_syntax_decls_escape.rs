use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Escapes all regular expression meta characters in `text`.
///
/// The string returned may be safely used as a literal in a regular
/// expression.
pub fn escape(text: &str) -> String {
    let mut quoted = String::new();
    escape_into(text, &mut quoted);
    quoted
}
