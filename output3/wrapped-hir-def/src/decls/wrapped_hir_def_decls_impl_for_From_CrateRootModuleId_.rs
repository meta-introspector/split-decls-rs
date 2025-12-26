use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<CrateRootModuleId> for ModuleDefId {
    fn from(value: CrateRootModuleId) -> Self {
        ModuleDefId::ModuleId(value.into())
    }
}
