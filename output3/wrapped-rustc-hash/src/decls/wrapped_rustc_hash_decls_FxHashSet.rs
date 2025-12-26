use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Type alias for a hash set that uses the Fx hashing algorithm.
#[cfg(feature = "std")]
pub type FxHashSet<V> = HashSet<V, FxBuildHasher>;
