use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Defines network byte order serialization.
///
/// Network byte order is defined by [RFC 1700][1] to be big-endian, and is
/// referred to in several protocol specifications.  This type is an alias of
/// [`BigEndian`].
///
/// [1]: https://tools.ietf.org/html/rfc1700
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, NetworkEndian, BigEndian};
///
/// let mut buf = [0; 2];
/// BigEndian::write_i16(&mut buf, -5_000);
/// assert_eq!(-5_000, NetworkEndian::read_i16(&buf));
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
pub type NetworkEndian = BigEndian;
