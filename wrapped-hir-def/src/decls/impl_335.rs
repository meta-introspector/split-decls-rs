macro_rules! deps {
    () => {
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl HasResolver for ImplId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { self . lookup (db) . container . resolver (db) . push_generic_params_scope (db , self . into ()) } }
    };
}

impl_335!()