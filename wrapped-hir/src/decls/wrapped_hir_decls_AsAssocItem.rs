use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait AsAssocItem {
    fn as_assoc_item(self, db: &dyn HirDatabase) -> Option<AssocItem>;
}
