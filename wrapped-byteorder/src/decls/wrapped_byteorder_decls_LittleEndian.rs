use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Defines little-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// LittleEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}
