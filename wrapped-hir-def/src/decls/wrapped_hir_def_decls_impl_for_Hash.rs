use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<N: AstIdNode> Hash for AssocItemLoc<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.container.hash(state);
        self.id.hash(state);
    }
}
