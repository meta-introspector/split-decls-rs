use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<N: AstIdNode> Clone for AssocItemLoc<N> {
    fn clone(&self) -> Self {
        *self
    }
}
