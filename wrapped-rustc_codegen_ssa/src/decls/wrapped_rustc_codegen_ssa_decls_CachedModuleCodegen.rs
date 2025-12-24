use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct CachedModuleCodegen {
    pub name: String,
    pub source: WorkProduct,
}
