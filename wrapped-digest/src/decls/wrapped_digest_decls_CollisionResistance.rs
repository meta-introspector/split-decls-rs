use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types with a certain collision resistance.
pub trait CollisionResistance {
    /// Collision resistance in bytes.
    ///
    /// This applies to an output size of at least `2 * CollisionResistance` bytes.
    /// For a smaller output size collision resistance can be usually calculated as
    /// `min(CollisionResistance, OutputSize / 2)`.
    type CollisionResistance: Unsigned;
}
