use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Encodes `data` as hex string using uppercase characters.
///
/// Apart from the characters' casing, this works exactly like `encode()`.
///
/// # Example
///
/// ```
/// assert_eq!(hex::encode_upper("Hello world!"), "48656C6C6F20776F726C6421");
/// assert_eq!(hex::encode_upper(vec![1, 2, 3, 15, 16]), "0102030F10");
/// ```
#[must_use]
#[cfg(feature = "alloc")]
pub fn encode_upper<T: AsRef<[u8]>>(data: T) -> String {
    let data = data.as_ref();
    let mut out = vec![0; data.len() * 2];
    encode_to_slice_upper(data, &mut out).unwrap();
    String::from_utf8(out).unwrap()
}
