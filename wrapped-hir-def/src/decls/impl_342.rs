macro_rules! deps {
    () => {
        DefDatabase!();
        Resolver!();
        HasResolver!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl HasResolver for EnumVariantId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { self . lookup (db) . parent . resolver (db) } }
    };
}

impl_342!()