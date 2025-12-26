use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'db> InstantiatedVariant<'db> {
    pub fn parent_enum(self, db: &dyn HirDatabase) -> InstantiatedEnum<'db> {
        InstantiatedEnum {
            inner: self.inner.id.lookup(db).parent.into(),
            args: self.args,
        }
    }
    pub fn fields(self, db: &dyn HirDatabase) -> Vec<InstantiatedField<'db>> {
        self.inner
            .id
            .fields(db)
            .fields()
            .iter()
            .map(|(id, _)| InstantiatedField {
                inner: Field {
                    parent: self.inner.into(),
                    id,
                },
                args: self.args,
            })
            .collect()
    }
}
