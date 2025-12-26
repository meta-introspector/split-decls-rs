use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Result when trying to finalize a hash.
#[derive(Debug)]
pub enum CollisionResult {
    /// No collision.
    Ok(Output<Sha1>),
    /// Collision occurred, but was mititgated.
    Mitigated(Output<Sha1>),
    /// Collision occurred, the hash is the one that collided.
    Collision(Output<Sha1>),
}
