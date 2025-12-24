use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustSourceWorkspaceConfig {
    CargoMetadata(CargoMetadataConfig),
    Json(ProjectJson),
}
