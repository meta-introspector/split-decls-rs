use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Encodes some bytes into a mutable slice of bytes using uppercase characters.
///
/// The output buffer, has to be able to hold exactly `input.len() * 2` bytes,
/// otherwise this function will return an error.
///
/// # Example
///
/// ```
/// # use hex::FromHexError;
/// # fn main() -> Result<(), FromHexError> {
/// let mut bytes = [0u8; 4 * 2];
///
/// hex::encode_to_slice_upper(b"kiwi", &mut bytes)?;
/// assert_eq!(&bytes, b"6B697769");
/// # Ok(())
/// # }
/// ```
pub fn encode_to_slice_upper<T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
) -> Result<(), FromHexError> {
    encode_to_slice_inner(input.as_ref(), output, HEX_CHARS_UPPER)
}
