use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if and only if the given character is a Unicode word
/// character.
///
/// A Unicode word character is defined by
/// [UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
/// In particular, a character
/// is considered a word character if it is in either of the `Alphabetic` or
/// `Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
/// or `Connector_Punctuation` general categories.
///
/// # Panics
///
/// If the `unicode-perl` feature is not enabled, then this function
/// panics. For this reason, it is recommended that callers use
/// [`try_is_word_character`] instead.
pub fn is_word_character(c: char) -> bool {
    try_is_word_character(c).expect("unicode-perl feature must be enabled")
}
