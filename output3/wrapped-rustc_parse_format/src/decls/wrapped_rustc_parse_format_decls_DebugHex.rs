use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum for the debug hex flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DebugHex {
    /// The `x` flag in `{:x?}`.
    Lower,
    /// The `X` flag in `{:X?}`.
    Upper,
}
