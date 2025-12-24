use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
impl<K, V, S> HashMapExt for std::collections::HashMap<K, V, S>
where
    S: BuildHasher + Default,
{
    fn new() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }
    fn with_capacity(capacity: usize) -> Self {
        std::collections::HashMap::with_capacity_and_hasher(capacity, S::default())
    }
}
