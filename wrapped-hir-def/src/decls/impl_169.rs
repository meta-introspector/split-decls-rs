macro_rules! deps {
    () => {
        VariantId!();
        ModuleId!();
        HasModule!();
        DefWithBodyId!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl HasModule for DefWithBodyId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match self { DefWithBodyId :: FunctionId (it) => it . module (db) , DefWithBodyId :: StaticId (it) => it . module (db) , DefWithBodyId :: ConstId (it) => it . module (db) , DefWithBodyId :: VariantId (it) => it . module (db) , } } }
    };
}

impl_169!()