macro_rules! deps {
    () => {
        DefDatabase!();
        VariantId!();
        HasResolver!();
        Resolver!();
        DefWithBodyId!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl HasResolver for DefWithBodyId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { DefWithBodyId :: ConstId (c) => c . resolver (db) , DefWithBodyId :: FunctionId (f) => f . resolver (db) , DefWithBodyId :: StaticId (s) => s . resolver (db) , DefWithBodyId :: VariantId (v) => v . resolver (db) , } } }
    };
}

impl_339!()