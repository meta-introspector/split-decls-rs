use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Mmap> for MmapRaw {
    fn from(value: Mmap) -> Self {
        Self { inner: value.inner }
    }
}
