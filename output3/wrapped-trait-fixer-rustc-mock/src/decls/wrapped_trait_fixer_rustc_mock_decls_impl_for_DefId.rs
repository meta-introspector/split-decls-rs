use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DefId {
    pub fn as_local(self) -> Option<LocalDefId> {
        Some(LocalDefId)
    }
    pub fn to_def_id(self) -> DefId {
        self
    }
}
