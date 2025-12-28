macro_rules! deps {
    () => {
        DefDatabase!();
        Resolver!();
        HasResolver!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl HasResolver for TypeAliasId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) . push_generic_params_scope (db , self . into ()) } }
    };
}

impl_334!()