use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Decodes a hex string into raw bytes.
///
/// Both, upper and lower case characters are valid in the input string and can
/// even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).
///
/// # Example
///
/// ```
/// assert_eq!(
///     hex::decode("48656c6c6f20776f726c6421"),
///     Ok("Hello world!".to_owned().into_bytes())
/// );
///
/// assert_eq!(hex::decode("123"), Err(hex::FromHexError::OddLength));
/// assert!(hex::decode("foo").is_err());
/// ```
#[cfg(feature = "alloc")]
pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError> {
    FromHex::from_hex(data)
}
