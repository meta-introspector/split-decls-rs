use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Escapes all regular expression meta characters in `pattern`.
///
/// The string returned may be safely used as a literal in a regular
/// expression.
pub fn escape(pattern: &str) -> alloc::string::String {
    regex_syntax::escape(pattern)
}
