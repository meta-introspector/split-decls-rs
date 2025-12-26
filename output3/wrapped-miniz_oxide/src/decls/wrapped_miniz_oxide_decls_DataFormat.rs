use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// How compressed data is wrapped.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]
#[non_exhaustive]
pub enum DataFormat {
    /// Wrapped using the [zlib](http://www.zlib.org/rfc-zlib.html) format.
    Zlib,
    /// Zlib wrapped but ignore and don't compute the adler32 checksum.
    /// Currently only used for inflate, behaves the same as Zlib for compression.
    ZLibIgnoreChecksum,
    /// Raw DEFLATE.
    Raw,
}
