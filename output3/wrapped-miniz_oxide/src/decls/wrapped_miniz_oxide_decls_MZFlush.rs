use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A list of flush types.
///
/// See <http://www.bolet.org/~pornin/deflate-flush.html> for more in-depth info.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "rustc-dep-of-std"), derive(Hash, Debug))]
pub enum MZFlush {
    /// Don't force any flushing.
    /// Used when more input data is expected.
    None = 0,
    /// Zlib partial flush.
    /// Currently treated as [`Sync`].
    Partial = 1,
    /// Finish compressing the currently buffered data, and output an empty raw block.
    /// Has no use in decompression.
    Sync = 2,
    /// Same as [`Sync`], but resets the compression dictionary so that further compressed
    /// data does not depend on data compressed before the flush.
    ///
    /// Has no use in decompression, and is an error to supply in that case.
    Full = 3,
    /// Attempt to flush the remaining data and end the stream.
    Finish = 4,
    /// Not implemented.
    Block = 5,
}
