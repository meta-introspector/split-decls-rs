use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum of alignments which are supported.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum Alignment {
    /// The value will be aligned to the left.
    AlignLeft,
    /// The value will be aligned to the right.
    AlignRight,
    /// The value will be aligned in the center.
    AlignCenter,
    /// The value will take on a default alignment.
    #[default]
    AlignUnknown,
}
