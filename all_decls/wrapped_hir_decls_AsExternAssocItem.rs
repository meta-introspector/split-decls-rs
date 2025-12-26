use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait AsExternAssocItem {
    fn as_extern_assoc_item(self, db: &dyn HirDatabase) -> Option<ExternAssocItem>;
}
