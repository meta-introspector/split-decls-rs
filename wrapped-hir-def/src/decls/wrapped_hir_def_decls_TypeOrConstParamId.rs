use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeOrConstParamId {
    pub parent: GenericDefId,
    pub local_id: LocalTypeOrConstParamId,
}
