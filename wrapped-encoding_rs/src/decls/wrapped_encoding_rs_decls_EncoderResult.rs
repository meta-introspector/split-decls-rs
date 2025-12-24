use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Result of a (potentially partial) encode operation without replacement.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum EncoderResult {
    /// The input was exhausted.
    ///
    /// If this result was returned from a call where `last` was `true`, the
    /// decoding process has completed. Otherwise, the caller should call a
    /// decode method again with more input.
    InputEmpty,
    /// The encoder cannot produce another unit of output, because the output
    /// buffer does not have enough space left.
    ///
    /// The caller must provide more output space upon the next call and re-push
    /// the remaining input to the decoder.
    OutputFull,
    /// The encoder encountered an unmappable character.
    ///
    /// The caller must either treat this as a fatal error or must append
    /// a placeholder to the output and then re-push the remaining input to the
    /// encoder.
    Unmappable(char),
}
