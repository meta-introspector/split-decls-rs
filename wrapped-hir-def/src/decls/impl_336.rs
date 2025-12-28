macro_rules! deps {
    () => {
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl HasResolver for ExternBlockId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
    };
}

impl_336!();