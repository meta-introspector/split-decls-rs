use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait HasVisibility {
    fn visibility(&self, db: &dyn HirDatabase) -> Visibility;
    fn is_visible_from(&self, db: &dyn HirDatabase, module: Module) -> bool {
        let vis = self.visibility(db);
        vis.is_visible_from(db, module.id)
    }
}
