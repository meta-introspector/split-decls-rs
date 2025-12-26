use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The version of the UUID, denoting the generating algorithm.
///
/// # References
///
/// * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
pub enum Version {
    /// The "nil" (all zeros) UUID.
    Nil = 0u8,
    /// Version 1: Timestamp and node ID.
    Mac = 1,
    /// Version 2: DCE Security.
    Dce = 2,
    /// Version 3: MD5 hash.
    Md5 = 3,
    /// Version 4: Random.
    Random = 4,
    /// Version 5: SHA-1 hash.
    Sha1 = 5,
    /// Version 6: Sortable Timestamp and node ID.
    SortMac = 6,
    /// Version 7: Timestamp and random.
    SortRand = 7,
    /// Version 8: Custom.
    Custom = 8,
    /// The "max" (all ones) UUID.
    Max = 0xff,
}
