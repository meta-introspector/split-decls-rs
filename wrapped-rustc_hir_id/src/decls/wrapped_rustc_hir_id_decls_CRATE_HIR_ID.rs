use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The `HirId` corresponding to `CRATE_NODE_ID` and `CRATE_DEF_ID`.
pub const CRATE_HIR_ID: HirId = HirId {
    owner: OwnerId { def_id: CRATE_DEF_ID },
    local_id: ItemLocalId::ZERO,
};
