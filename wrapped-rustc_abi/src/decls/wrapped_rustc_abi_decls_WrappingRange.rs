use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Inclusive wrap-around range of valid values, that is, if
/// start > end, it represents `start..=MAX`, followed by `0..=end`.
///
/// That is, for an i8 primitive, a range of `254..=2` means following
/// sequence:
///
///    254 (-2), 255 (-1), 0, 1, 2
///
/// This is intended specifically to mirror LLVM’s `!range` metadata semantics.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub struct WrappingRange {
    pub start: u128,
    pub end: u128,
}
