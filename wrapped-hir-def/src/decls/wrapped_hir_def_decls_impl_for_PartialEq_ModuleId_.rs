use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PartialEq<ModuleId> for CrateRootModuleId {
    fn eq(&self, other: &ModuleId) -> bool {
        other.block.is_none() && other.local_id == DefMap::ROOT
            && self.krate == other.krate
    }
}
