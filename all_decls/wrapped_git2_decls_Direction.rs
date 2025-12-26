use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration of the possible directions for a remote.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Data will be fetched (read) from this remote.
    Fetch,
    /// Data will be pushed (written) to this remote.
    Push,
}
