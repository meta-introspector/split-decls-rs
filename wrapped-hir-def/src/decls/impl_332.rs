macro_rules! deps {
    () => {
        Resolver!();
        HasResolver!();
        DefDatabase!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl HasResolver for ConstId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_332!();