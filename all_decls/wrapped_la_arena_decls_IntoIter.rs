use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator over the arena’s elements.
pub struct IntoIter<T>(Enumerate<<Vec<T> as IntoIterator>::IntoIter>);
