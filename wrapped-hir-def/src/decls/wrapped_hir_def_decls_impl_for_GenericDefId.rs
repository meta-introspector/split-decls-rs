use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl GenericDefId {
    pub fn file_id_and_params_of(
        self,
        db: &dyn DefDatabase,
    ) -> (HirFileId, Option<ast::GenericParamList>) {
        fn file_id_and_params_of_item_loc<Loc>(
            db: &dyn DefDatabase,
            def: impl Lookup<Database = dyn DefDatabase, Data = Loc>,
        ) -> (HirFileId, Option<ast::GenericParamList>)
        where
            Loc: src::HasSource,
            Loc::Value: ast::HasGenericParams,
        {
            let src = def.lookup(db).source(db);
            (src.file_id, ast::HasGenericParams::generic_param_list(&src.value))
        }
        match self {
            GenericDefId::FunctionId(it) => file_id_and_params_of_item_loc(db, it),
            GenericDefId::TypeAliasId(it) => file_id_and_params_of_item_loc(db, it),
            GenericDefId::AdtId(AdtId::StructId(it)) => {
                file_id_and_params_of_item_loc(db, it)
            }
            GenericDefId::AdtId(AdtId::UnionId(it)) => {
                file_id_and_params_of_item_loc(db, it)
            }
            GenericDefId::AdtId(AdtId::EnumId(it)) => {
                file_id_and_params_of_item_loc(db, it)
            }
            GenericDefId::TraitId(it) => file_id_and_params_of_item_loc(db, it),
            GenericDefId::ImplId(it) => file_id_and_params_of_item_loc(db, it),
            GenericDefId::ConstId(it) => (it.lookup(db).id.file_id, None),
            GenericDefId::StaticId(it) => (it.lookup(db).id.file_id, None),
        }
    }
    pub fn assoc_trait_container(self, db: &dyn DefDatabase) -> Option<TraitId> {
        match match self {
            GenericDefId::FunctionId(f) => f.lookup(db).container,
            GenericDefId::TypeAliasId(t) => t.lookup(db).container,
            GenericDefId::ConstId(c) => c.lookup(db).container,
            _ => return None,
        } {
            ItemContainerId::TraitId(trait_) => Some(trait_),
            _ => None,
        }
    }
    pub fn from_callable(db: &dyn DefDatabase, def: CallableDefId) -> GenericDefId {
        match def {
            CallableDefId::FunctionId(f) => f.into(),
            CallableDefId::StructId(s) => s.into(),
            CallableDefId::EnumVariantId(e) => e.lookup(db).parent.into(),
        }
    }
}
