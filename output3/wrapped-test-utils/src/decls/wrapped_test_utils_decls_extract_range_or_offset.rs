use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extracts `TextRange` or `TextSize` depending on the amount of `$0` markers
/// found in `text`.
///
/// # Panics
/// Panics if no `$0` marker is present in the `text`.
pub fn extract_range_or_offset(text: &str) -> (RangeOrOffset, String) {
    if let Some((range, text)) = try_extract_range(text) {
        return (RangeOrOffset::Range(range), text);
    }
    let (offset, text) = extract_offset(text);
    (RangeOrOffset::Offset(offset), text)
}
