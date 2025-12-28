macro_rules! deps {
    () => {
        HasResolver!();
        DefDatabase!();
        Resolver!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl HasResolver for ProcMacroId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_346!()