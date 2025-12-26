use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Calculates the number of positions in the two strings where the characters
/// differ. Returns an error if the strings have different lengths.
///
/// ```
/// use strsim::{hamming, StrSimError::DifferentLengthArgs};
///
/// assert_eq!(Ok(3), hamming("hamming", "hammers"));
///
/// assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
/// ```
pub fn hamming(a: &str, b: &str) -> HammingResult {
    generic_hamming(a.chars(), b.chars())
}
