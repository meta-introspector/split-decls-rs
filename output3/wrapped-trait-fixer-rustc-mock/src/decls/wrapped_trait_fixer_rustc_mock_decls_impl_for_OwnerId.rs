use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl OwnerId {
    pub const DUMMY: Self = Self;
    pub fn to_def_id(self) -> DefId {
        DefId
    }
}
