use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Adler-32 checksum calculator.
///
/// An instance of this type is equivalent to an Adler-32 checksum: It can be created in the default
/// state via [`new`] (or the provided `Default` impl), or from a precalculated checksum via
/// [`from_checksum`], and the currently stored checksum can be fetched via [`checksum`].
///
/// This type also implements `Hasher`, which makes it easy to calculate Adler-32 checksums of any
/// type that implements or derives `Hash`. This also allows using Adler-32 in a `HashMap`, although
/// that is not recommended (while every checksum is a hash function, they are not necessarily a
/// good one).
///
/// # Examples
///
/// Basic, piecewise checksum calculation:
///
/// ```
/// use adler2::Adler32;
///
/// let mut adler = Adler32::new();
///
/// adler.write_slice(&[0, 1, 2]);
/// adler.write_slice(&[3, 4, 5]);
///
/// assert_eq!(adler.checksum(), 0x00290010);
/// ```
///
/// Using `Hash` to process structures:
///
/// ```
/// use std::hash::Hash;
/// use adler2::Adler32;
///
/// #[derive(Hash)]
/// struct Data {
///     byte: u8,
///     word: u16,
///     big: u64,
/// }
///
/// let mut adler = Adler32::new();
///
/// let data = Data { byte: 0x1F, word: 0xABCD, big: !0 };
/// data.hash(&mut adler);
///
/// // hash value depends on architecture endianness
/// if cfg!(target_endian = "little") {
///     assert_eq!(adler.checksum(), 0x33410990);
/// }
/// if cfg!(target_endian = "big") {
///     assert_eq!(adler.checksum(), 0x331F0990);
/// }
///
/// ```
///
/// [`new`]: #method.new
/// [`from_checksum`]: #method.from_checksum
/// [`checksum`]: #method.checksum
#[derive(Debug, Copy, Clone)]
pub struct Adler32 {
    a: u16,
    b: u16,
}
