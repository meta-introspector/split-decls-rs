macro_rules! deps {
    () => {
        GenericDefId!();
        DefDatabase!();
        DefWithBodyId!();
        VariantId!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl DefWithBodyId { pub fn as_generic_def_id (self , db : & dyn DefDatabase) -> Option < GenericDefId > { match self { DefWithBodyId :: FunctionId (f) => Some (f . into ()) , DefWithBodyId :: StaticId (s) => Some (s . into ()) , DefWithBodyId :: ConstId (c) => Some (c . into ()) , DefWithBodyId :: VariantId (c) => Some (c . lookup (db) . parent . into ()) , } } }
    };
}

impl_705!()