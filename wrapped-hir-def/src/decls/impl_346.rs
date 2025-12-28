macro_rules! deps {
    () => {
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl HasResolver for ProcMacroId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_346!();