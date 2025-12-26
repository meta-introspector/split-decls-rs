use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "real_cargo_metadata"))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DummyMetadata;
