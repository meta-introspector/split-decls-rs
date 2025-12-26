use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A Adler-32 hash-able type.
pub trait Adler32Hash {
    /// Feeds this value into `Adler32`.
    fn hash(&self) -> u32;
}
