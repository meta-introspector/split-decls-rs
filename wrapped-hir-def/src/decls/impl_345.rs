macro_rules! deps {
    () => {
        Resolver!();
        DefDatabase!();
        HasResolver!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl HasResolver for Macro2Id { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_345!();