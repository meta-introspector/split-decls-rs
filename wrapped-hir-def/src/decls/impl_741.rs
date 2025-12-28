macro_rules! deps {
    () => {
        DefDatabase!();
        ModuleId!();
        DefWithBodyId!();
        HasModule!();
        VariantId!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl HasModule for DefWithBodyId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match self { DefWithBodyId :: FunctionId (it) => it . module (db) , DefWithBodyId :: StaticId (it) => it . module (db) , DefWithBodyId :: ConstId (it) => it . module (db) , DefWithBodyId :: VariantId (it) => it . module (db) , } } }
    };
}

impl_741!()