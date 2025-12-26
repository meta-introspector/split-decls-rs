use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Similar to rustc's `FxIndexMap`, `IndexMap` with extra
/// safety features added.
pub struct IndexMap<K, V> {
    index_map: fx::FxIndexMap<K, V>,
}
