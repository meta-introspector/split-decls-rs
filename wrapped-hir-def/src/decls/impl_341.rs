macro_rules! deps {
    () => {
        AdtId!();
        DefDatabase!();
        HasResolver!();
        GenericDefId!();
        Resolver!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl HasResolver for GenericDefId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { GenericDefId :: FunctionId (inner) => inner . resolver (db) , GenericDefId :: AdtId (adt) => adt . resolver (db) , GenericDefId :: TraitId (inner) => inner . resolver (db) , GenericDefId :: TypeAliasId (inner) => inner . resolver (db) , GenericDefId :: ImplId (inner) => inner . resolver (db) , GenericDefId :: ConstId (inner) => inner . resolver (db) , GenericDefId :: StaticId (inner) => inner . resolver (db) , } } }
    };
}

impl_341!()