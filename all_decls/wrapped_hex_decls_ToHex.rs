use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Encoding values as hex string.
///
/// This trait is implemented for all `T` which implement `AsRef<[u8]>`. This
/// includes `String`, `str`, `Vec<u8>` and `[u8]`.
///
/// # Example
///
/// ```
/// use hex::ToHex;
///
/// println!("{}", "Hello world!".encode_hex::<String>());
/// # assert_eq!("Hello world!".encode_hex::<String>(), "48656c6c6f20776f726c6421".to_string());
/// ```
///
/// *Note*: instead of using this trait, you might want to use [`encode()`].
pub trait ToHex {
    /// Encode the hex strict representing `self` into the result. Lower case
    /// letters are used (e.g. `f9b4ca`)
    fn encode_hex<T: iter::FromIterator<char>>(&self) -> T;
    /// Encode the hex strict representing `self` into the result. Upper case
    /// letters are used (e.g. `F9B4CA`)
    fn encode_hex_upper<T: iter::FromIterator<char>>(&self) -> T;
}
