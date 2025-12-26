use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Structure representing a gnuplot version number.
pub struct Version {
    /// The major version number
    pub major: usize,
    /// The minor version number
    pub minor: usize,
    /// The patch level
    pub patch: String,
}
