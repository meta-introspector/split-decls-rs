macro_rules! deps {
    () => {
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl HasResolver for Macro2Id { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_345!()