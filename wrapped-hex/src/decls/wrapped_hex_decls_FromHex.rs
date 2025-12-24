use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types that can be decoded from a hex string.
///
/// This trait is implemented for `Vec<u8>` and small `u8`-arrays.
///
/// # Example
///
/// ```
/// use core::str;
/// use hex::FromHex;
///
/// let buffer = <[u8; 12]>::from_hex("48656c6c6f20776f726c6421")?;
/// let string = str::from_utf8(&buffer).expect("invalid buffer length");
///
/// println!("{}", string); // prints "Hello world!"
/// # assert_eq!("Hello world!", string);
/// # Ok::<(), hex::FromHexError>(())
/// ```
pub trait FromHex: Sized {
    type Error;
    /// Creates an instance of type `Self` from the given hex string, or fails
    /// with a custom error type.
    ///
    /// Both, upper and lower case characters are valid and can even be
    /// mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>;
}
