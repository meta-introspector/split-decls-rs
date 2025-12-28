macro_rules! deps {
    () => {
        Resolver!();
        HasResolver!();
        DefDatabase!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl HasResolver for UseId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_338!();