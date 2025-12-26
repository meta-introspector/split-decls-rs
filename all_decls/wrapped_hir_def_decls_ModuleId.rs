use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId {
    krate: Crate,
    /// If this `ModuleId` was derived from a `DefMap` for a block expression, this stores the
    /// `BlockId` of that block expression. If `None`, this module is part of the crate-level
    /// `DefMap` of `krate`.
    block: Option<BlockId>,
    /// The module's ID in its originating `DefMap`.
    pub local_id: LocalModuleId,
}
