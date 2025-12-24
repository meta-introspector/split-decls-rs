use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternCrateDecl {
    pub(crate) id: ExternCrateId,
}
