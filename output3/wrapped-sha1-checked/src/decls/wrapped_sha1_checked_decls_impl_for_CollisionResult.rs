use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl CollisionResult {
    /// Returns the output hash.
    pub fn hash(&self) -> &Output<Sha1> {
        match self {
            CollisionResult::Ok(s) => s,
            CollisionResult::Mitigated(s) => s,
            CollisionResult::Collision(s) => s,
        }
    }
    /// Returns if there was a collision
    pub fn has_collision(&self) -> bool {
        !matches!(self, CollisionResult::Ok(_))
    }
}
