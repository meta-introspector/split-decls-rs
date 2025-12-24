use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl HasContainer for ExternBlock {
    fn container(&self, db: &dyn HirDatabase) -> ItemContainer {
        ItemContainer::Module(Module {
            id: self.id.lookup(db).container,
        })
    }
}
