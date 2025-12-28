macro_rules! deps {
    () => {
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl HasResolver for FunctionId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) . push_generic_params_scope (db , self . into ()) } }
    };
}

impl_331!()