use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Pre-allocated storage for a uniform data type
///
/// See the [module documentation] for more details.
///
/// [module documentation]: index.html
pub struct Slab<T> {
    entries: Vec<Entry<T>>,
    len: usize,
    next: usize,
}
