use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "real_cargo_metadata")]
impl AnyMetadata for cargo_metadata::Metadata {}
