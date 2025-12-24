use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ImplId {
    #[inline]
    pub fn impl_items(self, db: &dyn DefDatabase) -> &ImplItems {
        &self.impl_items_with_diagnostics(db).0
    }
    #[inline]
    pub fn impl_items_with_diagnostics(
        self,
        db: &dyn DefDatabase,
    ) -> &(ImplItems, DefDiagnostics) {
        ImplItems::of(db, self)
    }
}
