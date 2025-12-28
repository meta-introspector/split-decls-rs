macro_rules! deps {
    () => {
        AdtId!();
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl HasModule for AdtId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match * self { AdtId :: StructId (it) => it . module (db) , AdtId :: UnionId (it) => it . module (db) , AdtId :: EnumId (it) => it . module (db) , } } }
    };
}

impl_166!()