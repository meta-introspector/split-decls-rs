use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, PartialEq, Eq, Hash, Encodable, Decodable)]
pub struct OwnerId {
    pub def_id: LocalDefId,
}
