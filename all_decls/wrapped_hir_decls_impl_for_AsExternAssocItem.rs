use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AsExternAssocItem for TypeAlias {
    fn as_extern_assoc_item(self, db: &dyn HirDatabase) -> Option<ExternAssocItem> {
        as_extern_assoc_item(db, ExternAssocItem::TypeAlias, self.id)
    }
}
