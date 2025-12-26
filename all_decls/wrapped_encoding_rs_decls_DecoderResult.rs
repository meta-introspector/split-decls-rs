use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Result of a (potentially partial) decode operation without replacement.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum DecoderResult {
    /// The input was exhausted.
    ///
    /// If this result was returned from a call where `last` was `true`, the
    /// decoding process has completed. Otherwise, the caller should call a
    /// decode method again with more input.
    InputEmpty,
    /// The decoder cannot produce another unit of output, because the output
    /// buffer does not have enough space left.
    ///
    /// The caller must provide more output space upon the next call and re-push
    /// the remaining input to the decoder.
    OutputFull,
    /// The decoder encountered a malformed byte sequence.
    ///
    /// The caller must either treat this as a fatal error or must append one
    /// REPLACEMENT CHARACTER (U+FFFD) to the output and then re-push the
    /// the remaining input to the decoder.
    ///
    /// The first wrapped integer indicates the length of the malformed byte
    /// sequence. The second wrapped integer indicates the number of bytes
    /// that were consumed after the malformed sequence. If the second
    /// integer is zero, the last byte that was consumed is the last byte of
    /// the malformed sequence. Note that the malformed bytes may have been part
    /// of an earlier input buffer.
    ///
    /// The first wrapped integer can have values 1, 2, 3 or 4. The second
    /// wrapped integer can have values 0, 1, 2 or 3. The worst-case sum
    /// of the two is 6, which happens with ISO-2022-JP.
    Malformed(u8, u8),
}
