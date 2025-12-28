macro_rules! deps {
    () => {
        VariantId!();
        Resolver!();
        DefDatabase!();
        HasResolver!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl HasResolver for VariantId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { VariantId :: EnumVariantId (it) => it . resolver (db) , VariantId :: StructId (it) => it . resolver (db) , VariantId :: UnionId (it) => it . resolver (db) , } } }
    };
}

impl_343!();