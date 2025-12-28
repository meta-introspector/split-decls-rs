macro_rules! deps {
    () => {
        VariantId!();
        HasModule!();
        DefDatabase!();
        ModuleId!();
    };
}

macro_rules! impl_739 {
    () => {
        deps!();
        impl HasModule for VariantId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { VariantId :: EnumVariantId (it) => it . module (db) , VariantId :: StructId (it) => it . module (db) , VariantId :: UnionId (it) => it . module (db) , } } }
    };
}

impl_739!();