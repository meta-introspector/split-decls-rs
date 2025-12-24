use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct ItemLoc<N: AstIdNode> {
    pub container: ModuleId,
    pub id: AstId<N>,
}
