macro_rules! deps {
    () => {
        HasModule!();
        VariantId!();
        ModuleId!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl HasModule for VariantId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { VariantId :: EnumVariantId (it) => it . module (db) , VariantId :: StructId (it) => it . module (db) , VariantId :: UnionId (it) => it . module (db) , } } }
    };
}

impl_167!()