use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Normalize the string to avoid any unicode control characters.
///
/// This is important for untrusted input, as it can contain
/// invalid unicode sequences.
pub fn normalize_untrusted_str(s: &str) -> String {
    renderer::normalize_whitespace(s)
}
