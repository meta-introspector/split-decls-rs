use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Tracks the life cycle of a decoder from BOM sniffing to conversion to end.
#[derive(PartialEq, Debug, Copy, Clone)]
enum DecoderLifeCycle {
    /// The decoder has seen no input yet.
    AtStart,
    /// The decoder has seen no input yet but expects UTF-8.
    AtUtf8Start,
    /// The decoder has seen no input yet but expects UTF-16BE.
    AtUtf16BeStart,
    /// The decoder has seen no input yet but expects UTF-16LE.
    AtUtf16LeStart,
    /// The decoder has seen EF.
    SeenUtf8First,
    /// The decoder has seen EF, BB.
    SeenUtf8Second,
    /// The decoder has seen FE.
    SeenUtf16BeFirst,
    /// The decoder has seen FF.
    SeenUtf16LeFirst,
    /// Saw EF, BB but not BF, there was a buffer boundary after BB and the
    /// underlying decoder reported EF as an error, so we need to remember to
    /// push BB before the next buffer.
    ConvertingWithPendingBB,
    /// No longer looking for a BOM and EOF not yet seen.
    Converting,
    /// EOF has been seen.
    Finished,
}
