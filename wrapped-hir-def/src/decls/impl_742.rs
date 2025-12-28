macro_rules! deps {
    () => {
        DefDatabase!();
        GenericDefId!();
        ModuleId!();
        HasModule!();
        AdtId!();
    };
}

macro_rules! impl_742 {
    () => {
        deps!();
        impl HasModule for GenericDefId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match self { GenericDefId :: FunctionId (it) => it . module (db) , GenericDefId :: AdtId (it) => it . module (db) , GenericDefId :: TraitId (it) => it . module (db) , GenericDefId :: TypeAliasId (it) => it . module (db) , GenericDefId :: ImplId (it) => it . module (db) , GenericDefId :: ConstId (it) => it . module (db) , GenericDefId :: StaticId (it) => it . module (db) , } } }
    };
}

impl_742!()