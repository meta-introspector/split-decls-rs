use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Result of a (potentially partial) decode or encode operation with
/// replacement.
#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum CoderResult {
    /// The input was exhausted.
    ///
    /// If this result was returned from a call where `last` was `true`, the
    /// conversion process has completed. Otherwise, the caller should call a
    /// decode or encode method again with more input.
    InputEmpty,
    /// The converter cannot produce another unit of output, because the output
    /// buffer does not have enough space left.
    ///
    /// The caller must provide more output space upon the next call and re-push
    /// the remaining input to the converter.
    OutputFull,
}
