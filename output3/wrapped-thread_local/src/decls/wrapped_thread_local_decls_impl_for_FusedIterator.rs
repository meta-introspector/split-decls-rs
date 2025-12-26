use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Send> FusedIterator for IntoIter<T> {}
