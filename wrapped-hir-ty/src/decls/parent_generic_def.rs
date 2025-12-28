macro_rules! parent_generic_def {
    () => {
        pub (crate) fn parent_generic_def (db : & dyn DefDatabase , def : GenericDefId) -> Option < GenericDefId > { let container = match def { GenericDefId :: FunctionId (it) => it . lookup (db) . container , GenericDefId :: TypeAliasId (it) => it . lookup (db) . container , GenericDefId :: ConstId (it) => it . lookup (db) . container , GenericDefId :: StaticId (_) | GenericDefId :: AdtId (_) | GenericDefId :: TraitId (_) | GenericDefId :: ImplId (_) => return None , } ; match container { ItemContainerId :: ImplId (it) => Some (it . into ()) , ItemContainerId :: TraitId (it) => Some (it . into ()) , ItemContainerId :: ModuleId (_) | ItemContainerId :: ExternBlockId (_) => None , } }
    };
}

parent_generic_def!();