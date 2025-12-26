use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type FxIndexMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;
