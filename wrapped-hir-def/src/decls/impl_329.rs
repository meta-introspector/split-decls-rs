macro_rules! deps {
    () => {
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl HasResolver for TraitId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) . push_generic_params_scope (db , self . into ()) } }
    };
}

impl_329!()