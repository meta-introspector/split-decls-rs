use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for encoding binary data into text representations.
///
/// Implementors of this trait provide constant-time encoding operations
/// for a specific encoding format (Base64, Hex, etc.).
pub trait Encoder {
    /// Calculates the length of the encoded output for a given binary input length.
    ///
    /// # Arguments
    ///
    /// * `bin_len` - The length of the binary input in bytes
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The required length for the encoded output
    /// * `Err(Error::Overflow)` - If the calculation would overflow
    fn encoded_len(bin_len: usize) -> Result<usize, Error>;
    /// Encodes binary data into a text representation.
    ///
    /// # Arguments
    ///
    /// * `encoded` - Mutable buffer to store the encoded output
    /// * `bin` - Binary input data to encode
    ///
    /// # Returns
    ///
    /// * `Ok(&[u8])` - A slice of the encoded buffer containing the encoded data
    /// * `Err(Error::Overflow)` - If the output buffer is too small
    fn encode<IN: AsRef<[u8]>>(encoded: &mut [u8], bin: IN) -> Result<&[u8], Error>;
    /// Encodes binary data and returns the result as a string slice.
    ///
    /// # Arguments
    ///
    /// * `encoded` - Mutable buffer to store the encoded output
    /// * `bin` - Binary input data to encode
    ///
    /// # Returns
    ///
    /// * `Ok(&str)` - A string slice containing the encoded data
    /// * `Err(Error::Overflow)` - If the output buffer is too small
    fn encode_to_str<IN: AsRef<[u8]>>(encoded: &mut [u8], bin: IN) -> Result<&str, Error> {
        Ok(core::str::from_utf8(Self::encode(encoded, bin)?).unwrap())
    }
    /// Encodes binary data and returns the result as a String.
    ///
    /// This method is only available when the `std` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `bin` - Binary input data to encode
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - A String containing the encoded data
    /// * `Err(Error::Overflow)` - If the calculation would overflow
    #[cfg(feature = "std")]
    fn encode_to_string<IN: AsRef<[u8]>>(bin: IN) -> Result<String, Error> {
        let mut encoded = vec![0u8; Self::encoded_len(bin.as_ref().len())?];
        let encoded_len = Self::encode(&mut encoded, bin)?.len();
        encoded.truncate(encoded_len);
        Ok(String::from_utf8(encoded).unwrap())
    }
}
