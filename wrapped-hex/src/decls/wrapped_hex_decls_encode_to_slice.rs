use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Encodes some bytes into a mutable slice of bytes using lowercase characters.
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
/// hex::encode_to_slice(b"kiwi", &mut bytes)?;
/// assert_eq!(&bytes, b"6b697769");
/// # Ok(())
/// # }
/// ```
///
/// If the buffer is too large, an error is returned:
///
/// ```
/// use hex::FromHexError;
/// # fn main() -> Result<(), FromHexError> {
/// let mut bytes = [0_u8; 5 * 2];
///
/// assert_eq!(hex::encode_to_slice(b"kiwi", &mut bytes), Err(FromHexError::InvalidStringLength));
///
/// // you can do this instead:
/// hex::encode_to_slice(b"kiwi", &mut bytes[..4 * 2])?;
/// assert_eq!(&bytes, b"6b697769\0\0");
/// # Ok(())
/// # }
/// ```
pub fn encode_to_slice<T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
) -> Result<(), FromHexError> {
    encode_to_slice_inner(input.as_ref(), output, HEX_CHARS_LOWER)
}
