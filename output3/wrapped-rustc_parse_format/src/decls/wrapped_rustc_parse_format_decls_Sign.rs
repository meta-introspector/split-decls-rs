use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum for the sign flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sign {
    /// The `+` flag.
    Plus,
    /// The `-` flag.
    Minus,
}
