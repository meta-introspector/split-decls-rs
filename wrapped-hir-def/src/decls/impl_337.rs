macro_rules! deps {
    () => {
        Resolver!();
        HasResolver!();
        DefDatabase!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl HasResolver for ExternCrateId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_337!()