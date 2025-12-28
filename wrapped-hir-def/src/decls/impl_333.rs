macro_rules! deps {
    () => {
        HasResolver!();
        DefDatabase!();
        Resolver!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl HasResolver for StaticId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_333!();