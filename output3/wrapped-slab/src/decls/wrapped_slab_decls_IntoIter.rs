use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A consuming iterator over the values stored in a `Slab`
pub struct IntoIter<T> {
    entries: iter::Enumerate<vec::IntoIter<Entry<T>>>,
    len: usize,
}
