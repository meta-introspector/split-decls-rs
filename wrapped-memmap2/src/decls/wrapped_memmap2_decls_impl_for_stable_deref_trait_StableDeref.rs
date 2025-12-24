use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "stable_deref_trait")]
unsafe impl stable_deref_trait::StableDeref for MmapMut {}
