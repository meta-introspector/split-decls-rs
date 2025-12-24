use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents the generated patches, grouped by their repository URL.
/// The key is the repository URL (e.g., "https://github.com/meta-introspector/time-rs").
/// The value is a vector of `PatchEntry` for that repository.
pub type GeneratedPatches = HashMap<String, Vec<PatchEntry>>;
