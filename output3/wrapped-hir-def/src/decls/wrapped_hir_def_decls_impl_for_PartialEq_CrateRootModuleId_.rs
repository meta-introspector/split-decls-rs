use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PartialEq<CrateRootModuleId> for ModuleId {
    fn eq(&self, other: &CrateRootModuleId) -> bool {
        other == self
    }
}
