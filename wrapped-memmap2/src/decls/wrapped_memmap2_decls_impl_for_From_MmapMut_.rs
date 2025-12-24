use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<MmapMut> for MmapRaw {
    fn from(value: MmapMut) -> Self {
        Self { inner: value.inner }
    }
}
