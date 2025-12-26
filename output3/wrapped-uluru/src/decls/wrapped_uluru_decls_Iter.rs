use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Iterator over values in an [`LRUCache`], from most-recently-used to least-recently-used.
pub struct Iter<'a, T, const N: usize> {
    cache: &'a LRUCache<T, N>,
    pos: u16,
}
