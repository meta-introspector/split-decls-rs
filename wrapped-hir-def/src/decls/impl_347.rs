macro_rules! deps {
    () => {
        DefDatabase!();
        HasResolver!();
        Resolver!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl HasResolver for MacroRulesId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_347!();