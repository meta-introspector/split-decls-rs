macro_rules! deps {
    () => {
        DefDatabase!();
        CallableDefId!();
    };
}

macro_rules! impl_716 {
    () => {
        deps!();
        impl CallableDefId { pub fn krate (self , db : & dyn DefDatabase) -> Crate { match self { CallableDefId :: FunctionId (f) => f . krate (db) , CallableDefId :: StructId (s) => s . krate (db) , CallableDefId :: EnumVariantId (e) => e . krate (db) , } } }
    };
}

impl_716!();