use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Defines big-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 4];
/// BigEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, BigEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BigEndian {}
