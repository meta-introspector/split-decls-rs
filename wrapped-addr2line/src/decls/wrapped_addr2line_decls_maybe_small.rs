use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "smallvec"))]
mod maybe_small {
    pub type Vec<T> = alloc::vec::Vec<T>;
    pub type IntoIter<T> = alloc::vec::IntoIter<T>;
}
