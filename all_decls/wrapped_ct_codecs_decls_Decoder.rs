use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for decoding text representations back into binary data.
///
/// Implementors of this trait provide constant-time decoding operations
/// for a specific encoding format (Base64, Hex, etc.).
pub trait Decoder {
    /// Decodes text data back into its binary representation.
    ///
    /// # Arguments
    ///
    /// * `bin` - Mutable buffer to store the decoded output
    /// * `encoded` - Text input data to decode
    /// * `ignore` - Optional set of characters to ignore during decoding (e.g., whitespace)
    ///
    /// # Returns
    ///
    /// * `Ok(&[u8])` - A slice of the binary buffer containing the decoded data
    /// * `Err(Error::Overflow)` - If the output buffer is too small
    /// * `Err(Error::InvalidInput)` - If the input contains invalid characters
    fn decode<'t, IN: AsRef<[u8]>>(
        bin: &'t mut [u8],
        encoded: IN,
        ignore: Option<&[u8]>,
    ) -> Result<&'t [u8], Error>;
    /// Decodes text data and returns the result as a Vec<u8>.
    ///
    /// This method is only available when the `std` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `encoded` - Text input data to decode
    /// * `ignore` - Optional set of characters to ignore during decoding (e.g., whitespace)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - A Vec containing the decoded binary data
    /// * `Err(Error::InvalidInput)` - If the input contains invalid characters
    #[cfg(feature = "std")]
    fn decode_to_vec<IN: AsRef<[u8]>>(
        encoded: IN,
        ignore: Option<&[u8]>,
    ) -> Result<Vec<u8>, Error> {
        let mut bin = vec![0u8; encoded.as_ref().len()];
        let bin_len = Self::decode(&mut bin, encoded, ignore)?.len();
        bin.truncate(bin_len);
        Ok(bin)
    }
}
